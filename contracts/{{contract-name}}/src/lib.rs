use near_plugins::{
    access_control, access_control_any, pause, AccessControlRole, AccessControllable, Pausable,
    Upgradable,
};
use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    env, near,
    serde::{Deserialize, Serialize},
    BorshStorageKey, PanicOnDefault,
};

mod errors;
mod upgrade;

pub use errors::*;
use once_cell::sync::Lazy;

pub type ContractResult<T> = Result<T, ContractError>;

pub static LAZY_CONST: Lazy<String> = Lazy::new(|| {
    #[cfg(feature = "test")]
    {
        "testnet".to_string()
    }

    #[cfg(not(feature = "test"))]
    "near".to_string()
});

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
enum StorageKey {
    Keys,
}

#[near(serializers = [borsh])]
pub struct ContractData {}

#[near(serializers = [borsh])]
pub enum VersionedContractData {
    Current(ContractData),
}

#[derive(AccessControlRole, Deserialize, Serialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    DAO,
    PauseManager,
    UnpauseManager,
    UpgradableCodeStager,
    UpgradableCodeDeployer,
}

#[near(contract_state)]
#[derive(Pausable, Upgradable, PanicOnDefault)]
#[access_control(role_type(Role))]
#[pausable(pause_roles(Role::PauseManager), unpause_roles(Role::UnpauseManager))]
#[upgradable(access_control_roles(
    code_stagers(Role::UpgradableCodeStager, Role::DAO),
    code_deployers(Role::UpgradableCodeDeployer, Role::DAO),
    duration_initializers(Role::DAO),
    duration_update_stagers(Role::DAO),
    duration_update_appliers(Role::DAO),
))]
pub struct Contract {
    data: VersionedContractData,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {
            data: VersionedContractData::Current(ContractData {}),
        };
        contract.acl_init_super_admin(env::predecessor_account_id());
        contract.acl_grant_role(Role::DAO.into(), env::predecessor_account_id());
        contract.acl_grant_role(Role::PauseManager.into(), env::predecessor_account_id());
        contract.acl_grant_role(Role::UnpauseManager.into(), env::predecessor_account_id());
        contract
    }

    #[handle_result(aliased)]
    #[payable]
    #[access_control_any(roles(Role::DAO))]
    #[pause(except(roles(Role::DAO)))]
    pub fn near_plugins_usage(&mut self) -> ContractResult<()> {
        Ok(())
    }
}

impl Contract {
    #[allow(unreachable_patterns)]
    fn data(&self) -> &ContractData {
        match &self.data {
            VersionedContractData::Current(data) => data,
            _ => unimplemented!(),
        }
    }

    #[allow(unreachable_patterns)]
    fn data_mut(&mut self) -> &mut ContractData {
        match &mut self.data {
            VersionedContractData::Current(data) => data,
            _ => unimplemented!(),
        }
    }
}
