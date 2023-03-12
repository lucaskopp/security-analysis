use crate::helper_functions::api;
use crate::helper_structs::{
    BalanceSheetStatement, CashFlowStatement, FetchStats, IncomeStatement, Profile, TimePeriod,
};
use crate::utils::{needs_update_based_on_time, update_pull_stats};
use serde::de::DeserializeOwned;

use core::fmt::Debug;
use std::any::TypeId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Other {
    pub profile: Vec<Profile>,
}

impl Other {
    pub fn new() -> Self {
        Self {
            profile: vec![],
        }
    }

    pub async fn fetch<T>(&mut self, symbol: &String)
    where
        T: DeserializeOwned + Debug + 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<Profile>() {
            self.profile = api::<Profile>(&TimePeriod::NA(), symbol, "".to_string()).await;
        }
    }
}
