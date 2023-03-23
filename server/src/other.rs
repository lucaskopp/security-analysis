use crate::helper_functions::api;
use crate::helper_structs::{AdvancedLeveredDiscountedCashFlow, Profile, TimePeriod};
use serde::de::DeserializeOwned;

use core::fmt::Debug;
use std::any::TypeId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Other {
    pub profile: Vec<Profile>,
    pub dcf: Vec<AdvancedLeveredDiscountedCashFlow>,
}

impl Other {
    pub fn new() -> Self {
        Self {
            profile: vec![],
            dcf: vec![],
        }
    }

    pub async fn fetch<T>(&mut self, symbol: &String)
    where
        T: DeserializeOwned + Debug + 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<Profile>() {
            self.profile = api::<Profile>(&TimePeriod::NA(), symbol, "".to_string()).await;
        } else if TypeId::of::<T>() == TypeId::of::<AdvancedLeveredDiscountedCashFlow>() {
            self.dcf = api::<AdvancedLeveredDiscountedCashFlow>(&TimePeriod::NA(), symbol, "".to_string()).await;
        }
    }
}
