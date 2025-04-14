use futures::stream::BoxStream;
use primitive_types::H256;
use std::cell::RefCell;
use std::num::NonZeroUsize;
use std::sync::Mutex;

use crate::types::RequestStatus;

use super::{types, L1Error, L1Interface};

pub struct CachedL1Interface<L1> {
    l1: L1,
    update_cache: Mutex<RefCell<lru::LruCache<(u128, u128), types::abi::L1Update>>>,
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
    async fn subscribe_deposit(& self, subscription: crate::Subscription) -> Result<BoxStream<u128>, L1Error>{
        self.l1.subscribe_deposit(subscription).await
    }

    async fn subscribe_new_batch(&self, subscription: crate::Subscription) -> Result<BoxStream<(H256, (u128, u128))>, L1Error>{
        self.l1.subscribe_new_batch(subscription).await
    }

    async fn get_deposit(&self, request_id: u128) -> Result<Option<types::Deposit>, L1Error> {
        self.l1.get_deposit(request_id).await
    }

    async fn erc20_balance(&self, token: [u8; 20], account: [u8; 20]) -> Result<u128, L1Error> {
        self.l1.erc20_balance(token, account).await
    }

    async fn native_balance(&self, account: [u8; 20]) -> Result<u128, L1Error> {
        self.l1.native_balance(account).await
    }

    async fn get_merkle_root(
        &self,
        request_id: u128,
    ) -> Result<Option<([u8; 32], (u128, u128))>, L1Error> {
        self.l1.get_merkle_root(request_id).await
    }

    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error> {
        self.l1.get_latest_reqeust_id().await
    }

    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error> {
        self.l1.get_latest_finalized_request_id().await
    }

    async fn get_update(&self, start: u128, end: u128) -> Result<types::abi::L1Update, L1Error> {
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

    async fn close_cancel(
        &self,
        cancel: gasp_types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        self.l1.close_cancel(cancel, merkle_root, proof).await
    }

    async fn close_withdrawal(
        &self,
        withdrawal: gasp_types::Withdrawal,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        self.l1
            .close_withdrawal(withdrawal, merkle_root, proof)
            .await
    }

    async fn ferry_withdrawal(&self, withdrawal: gasp_types::Withdrawal) -> Result<H256, L1Error> {
        self.l1.ferry_withdrawal(withdrawal).await
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
    async fn get_status(&self, request_hash: H256) -> Result<crate::types::RequestStatus, L1Error> {
        let cached = {
            let guard = self.is_closed_cache.lock().expect("poison");
            let cached = guard.borrow_mut().get(&request_hash).cloned();
            cached
        };

        match cached {
            Some(_cached) => Ok(RequestStatus::Closed),
            None => {
                let status = self.l1.get_status(request_hash).await?;
                if let RequestStatus::Closed = &status {
                    let guard = self.is_closed_cache.lock().expect("poison");
                    let _ = guard.borrow_mut().put(request_hash, true);
                }
                Ok(status)
            }
        }
    }

    //TODO: fix
    // #[tracing::instrument(skip(self))]
    // async fn get_native_balance(&self, address: [u8; 20]) -> Result<u128, L1Error> {
    //     self.l1.get_native_balance(address).await
    // }
}
