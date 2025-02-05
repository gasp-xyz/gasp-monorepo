use primitive_types::H256;
use std::cell::RefCell;
use std::num::NonZeroUsize;
use std::sync::Mutex;

use super::{types, L1Error, L1Interface};

pub struct CachedL1Interface<L1> {
    l1: L1,
    update_cache: Mutex<RefCell<lru::LruCache<(u128, u128), types::L1Update>>>,
    update_hash_cache: Mutex<RefCell<lru::LruCache<(u128, u128), H256>>>,
    is_closed_cache: Mutex<RefCell<lru::LruCache<H256, bool>>>,
}

impl<L1: L1Interface> CachedL1Interface<L1> {
    pub fn new(l1: L1, capacity: NonZeroUsize) -> Self {
        Self {
            l1,
            update_cache: Mutex::new(RefCell::new(lru::LruCache::new(capacity))),
            update_hash_cache: Mutex::new(RefCell::new(lru::LruCache::new(capacity))),
            is_closed_cache: Mutex::new(RefCell::new(lru::LruCache::new(capacity))),
        }
    }
}

impl<L1> L1Interface for CachedL1Interface<L1>
where
    L1: L1Interface,
{
    fn account_address(&self) -> [u8; 20] {
        self.l1.account_address()
    }

    async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error> {
        self.l1.get_merkle_root(request_id).await
    }

    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error> {
        self.l1.get_latest_reqeust_id().await
    }

    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error> {
        self.l1.get_latest_finalized_request_id().await
    }

    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error> {
        let cached = {
            let guard = self.update_cache.lock().expect("poison");
            let cached = guard.borrow_mut().get(&(start, end)).cloned();
            cached
        };

        match cached {
            Some(cached) => Ok(cached),
            None => {
                let update = self.l1.get_update(start, end).await?;
                {
                    let guard = self.update_cache.lock().expect("poison");
                    let _ = guard.borrow_mut().put((start, end), update.clone());
                }
                Ok(update)
            }
        }
    }

    async fn estimate_gas_in_wei(&self) -> Result<(u128, u128), L1Error> {
        self.l1.estimate_gas_in_wei().await
    }

    async fn close_cancel(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        self.l1.close_cancel(cancel, merkle_root, proof).await
    }

    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error> {
        let cached = {
            let guard = self.update_hash_cache.lock().expect("poison");
            let cached = guard.borrow_mut().get(&(start, end)).cloned();
            cached
        };

        match cached {
            Some(cached) => Ok(cached),
            None => {
                let update_hash = self.l1.get_update_hash(start, end).await?;
                {
                    let guard = self.update_hash_cache.lock().expect("poison");
                    let _ = guard.borrow_mut().put((start, end), update_hash);
                }
                Ok(update_hash)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn is_closed(&self, request_hash: H256) -> Result<bool, L1Error> {
        let cached = {
            let guard = self.is_closed_cache.lock().expect("poison");
            let cached = guard.borrow_mut().get(&request_hash).cloned();
            cached
        };

        match cached {
            Some(cached) => Ok(cached),
            None => {
                let is_closed = self.l1.is_closed(request_hash).await?;

                if is_closed {
                    let guard = self.is_closed_cache.lock().expect("poison");
                    let _ = guard.borrow_mut().put(request_hash, true);
                }
                Ok(is_closed)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_native_balance(&self, address: [u8; 20]) -> Result<u128, L1Error> {
        self.l1.get_native_balance(address).await
    }
}
