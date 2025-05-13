use crate::{utils::simulate_send_and_wait_for_result, L1Error};
use alloy::{
    network::Network,
    providers::{Provider, WalletProvider},
    sol,
    sol_types::SolValue,
    transports::Transport,
};
use gasp_types::H256;

mod abi {
    use super::*;

    sol! {
        enum Origin {
            L1,
            L2
        }

        struct RequestId {
            Origin origin;
            uint256 id;
        }

        struct Withdrawal {
            RequestId requestId;
            address recipient;
            address tokenAddress;
            uint256 amount;
            uint256 ferryTip;
        }

        struct Range {
            uint256 start;
            uint256 end;
        }


        #[sol(rpc)]
       contract CickaContract {

        function closeWithdrawalBatch(
            Withdrawal[] calldata withdrawals,
            bytes32[] calldata merkleRoots,
            bytes32[][] calldata proofs
        ) external;
       }
    }
}

#[derive(Clone)]
pub struct Cicka<T, P, N> {
    contract_handle: abi::CickaContract::CickaContractInstance<T, P, N>,
    address: [u8; 20],
}

impl<T, P, N> Cicka<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    pub fn address(&self) -> [u8; 20] {
        self.address
    }

    pub async fn send_close_withdrawals(
        &self,
        withdrawals: Vec<(gasp_types::Withdrawal, H256, Vec<H256>)>,
    ) -> Result<H256, L1Error> {
        let (withdrawals, roots, proofs) = withdrawals.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut withdrawals, mut roots, mut proofs), (w, r, p)| {
                let abi_proofs: Vec<alloy::primitives::FixedBytes<32>> = p
                    .into_iter()
                    .map(|elem| elem.to_fixed_bytes().into())
                    .collect();

                let abi_compatible_withdrawal: crate::types::abi::Withdrawal = w.into();
                let w = abi::Withdrawal::abi_decode(&abi_compatible_withdrawal.abi_encode(), true);

                withdrawals.push(w);
                roots.push(r.to_fixed_bytes().into());
                proofs.push(abi_proofs);
                (withdrawals, roots, proofs)
            },
        );
        let withdrawals = withdrawals
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| L1Error::DeserializationError)?;
        let call = self
            .contract_handle
            .closeWithdrawalBatch(withdrawals, roots, proofs);

        simulate_send_and_wait_for_result::<T, P, _, N>(self.contract_handle.provider(), call).await
    }

    pub fn new(provider: P, address: [u8; 20]) -> Self {
        Self {
            contract_handle: abi::CickaContract::CickaContractInstance::new(
                address.into(),
                provider,
            ),
            address,
        }
    }
}
