use ethereum_rust_storage::{error::StoreError, Store};
use serde_json::Value;
use std::fmt::Display;
use tracing::info;

use crate::{utils::RpcErr, RpcHandler};
use ethereum_rust_core::{types::BlockNumber, Address, BigEndianHash, H256};

use super::block::BlockIdentifier;
use ethereum_rust_core::types::BlockHash;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum BlockIdentifierOrHash {
    Hash(BlockHash),
    Identifier(BlockIdentifier),
}

impl BlockIdentifierOrHash {
    #[allow(unused)]
    pub fn resolve_block_number(&self, storage: &Store) -> Result<Option<BlockNumber>, StoreError> {
        match self {
            BlockIdentifierOrHash::Identifier(id) => id.resolve_block_number(storage),
            BlockIdentifierOrHash::Hash(block_hash) => storage.get_block_number(*block_hash),
        }
    }
}

pub struct GetBalanceRequest {
    pub address: Address,
    pub block: BlockIdentifierOrHash,
}

pub struct GetCodeRequest {
    pub address: Address,
    pub block: BlockIdentifierOrHash,
}

pub struct GetStorageAtRequest {
    pub address: Address,
    pub storage_slot: H256,
    pub block: BlockIdentifierOrHash,
}

impl RpcHandler for GetBalanceRequest {
    fn parse(params: &Option<Vec<Value>>) -> Option<GetBalanceRequest> {
        let params = params.as_ref()?;
        if params.len() != 2 {
            return None;
        };
        Some(GetBalanceRequest {
            address: serde_json::from_value(params[0].clone()).ok()?,
            block: serde_json::from_value(params[1].clone()).ok()?,
        })
    }
    fn handle(&self, storage: Store) -> Result<Value, RpcErr> {
        info!(
            "Requested balance of account {} at block {}",
            self.address, self.block
        );
        let account = storage.get_account_info(self.address)?;
        let balance = account.map(|acc| acc.balance).unwrap_or_default();

        serde_json::to_value(format!("{:#x}", balance)).map_err(|_| RpcErr::Internal)
    }
}

impl RpcHandler for GetCodeRequest {
    fn parse(params: &Option<Vec<Value>>) -> Option<GetCodeRequest> {
        let params = params.as_ref()?;
        if params.len() != 2 {
            return None;
        };
        Some(GetCodeRequest {
            address: serde_json::from_value(params[0].clone()).ok()?,
            block: serde_json::from_value(params[1].clone()).ok()?,
        })
    }
    fn handle(&self, storage: Store) -> Result<Value, RpcErr> {
        info!(
            "Requested code of account {} at block {}",
            self.address, self.block
        );
        let code = storage
            .get_code_by_account_address(self.address)?
            .unwrap_or_default();

        serde_json::to_value(format!("0x{:x}", code)).map_err(|_| RpcErr::Internal)
    }
}

impl RpcHandler for GetStorageAtRequest {
    fn parse(params: &Option<Vec<Value>>) -> Option<GetStorageAtRequest> {
        let params = params.as_ref()?;
        if params.len() != 3 {
            return None;
        };
        Some(GetStorageAtRequest {
            address: serde_json::from_value(params[0].clone()).ok()?,
            storage_slot: serde_json::from_value(params[1].clone()).ok()?,
            block: serde_json::from_value(params[2].clone()).ok()?,
        })
    }
    fn handle(&self, storage: Store) -> Result<Value, RpcErr> {
        info!(
            "Requested storage sot {} of account {} at block {}",
            self.storage_slot, self.address, self.block
        );
        let storage_value = storage
            .get_storage_at(self.address, self.storage_slot)?
            .unwrap_or_default();
        let storage_value = H256::from_uint(&storage_value);

        serde_json::to_value(format!("{:#x}", storage_value)).map_err(|_| RpcErr::Internal)
    }
}

impl Display for BlockIdentifierOrHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockIdentifierOrHash::Identifier(id) => id.fmt(f),
            BlockIdentifierOrHash::Hash(hash) => hash.fmt(f),
        }
    }
}
