use crate::helper_functions::api;
use crate::helper_structs::{
    BalanceSheetStatement, CashFlowStatement, FetchStats, IncomeStatement, TimePeriod,
};
use crate::utils::{needs_update_based_on_time, update_pull_stats};
use serde::de::DeserializeOwned;

use core::fmt::Debug;
use std::any::TypeId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Statements {
    pub annual_income: (Vec<IncomeStatement>, FetchStats),
    pub quarter_income: (Vec<IncomeStatement>, FetchStats),
    pub annual_balance: (Vec<BalanceSheetStatement>, FetchStats),
    pub quarter_balance: (Vec<BalanceSheetStatement>, FetchStats),
    pub annual_cash: (Vec<CashFlowStatement>, FetchStats),
    pub quarter_cash: (Vec<CashFlowStatement>, FetchStats),
}

impl Statements {
    pub fn new() -> Self {
        Self {
            annual_balance: (Vec::new(), FetchStats::new(0)),
            quarter_balance: (Vec::new(), FetchStats::new(0)),
            annual_income: (Vec::new(), FetchStats::new(0)),
            quarter_income: (Vec::new(), FetchStats::new(0)),
            annual_cash: (Vec::new(), FetchStats::new(0)),
            quarter_cash: (Vec::new(), FetchStats::new(0)),
        }
    }

    pub async fn fetch<T>(&mut self, period: TimePeriod, symbol: &String)
    where
        T: DeserializeOwned + Debug + 'static,
    {
        let should_update = needs_update_based_on_time(self, &period);
        let stats = update_pull_stats(&period);

        if TypeId::of::<T>() == TypeId::of::<IncomeStatement>() {
            match period {
                TimePeriod::Annual(v) => {
                    if (self.annual_income.0.len() < v as usize
                        && v > self.annual_income.1.last_pull_length as u8)
                        || should_update
                    {
                        self.annual_income = (
                            api::<IncomeStatement>(&period, &symbol, "".to_string()).await,
                            stats,
                        );
                    } 
                    // else {
                    //     println!("USED CACHE FOR INCOME - {}!", &symbol);
                    // }
                }
                TimePeriod::Quarter(v) => {
                    if (self.quarter_income.0.len() < v as usize
                        && v > self.quarter_income.1.last_pull_length as u8)
                        || should_update
                    {
                        self.quarter_income = (
                            api::<IncomeStatement>(&period, &symbol, "".to_string()).await,
                            stats,
                        );
                    } 
                    // else {
                    //     println!("USED CACHE FOR INCOME - {} (QTR)!", &symbol);
                    // }
                }
                _ => {}
            }
        } else if TypeId::of::<T>() == TypeId::of::<BalanceSheetStatement>() {
            match period {
                TimePeriod::Annual(v) => {
                    if (self.annual_balance.0.len() < v as usize
                        && v > self.annual_balance.1.last_pull_length as u8)
                        || should_update
                    {
                        self.annual_balance = (
                            api::<BalanceSheetStatement>(&period, &symbol, "".to_string()).await,
                            stats,
                        );
                    } 
                    // else {
                    //     println!("USED CACHE FOR BALANCE - {}!", &symbol);
                    // }
                }
                TimePeriod::Quarter(v) => {
                    if (self.quarter_balance.0.len() < v as usize
                        && v > self.quarter_balance.1.last_pull_length as u8)
                        || should_update
                    {
                        self.quarter_balance = (
                            api::<BalanceSheetStatement>(&period, &symbol, "".to_string()).await,
                            stats,
                        );
                    } 
                    // else {
                    //     println!("USED CACHE FOR BALANCE - {} (QTR)!", &symbol);
                    // }
                }
                _ => {}
            }
        } else if TypeId::of::<T>() == TypeId::of::<CashFlowStatement>() {
            match period {
                TimePeriod::Annual(v) => {
                    if (self.annual_cash.0.len() < v as usize
                        && v > self.annual_cash.1.last_pull_length as u8)
                        || should_update
                    {
                        self.annual_cash = (
                            api::<CashFlowStatement>(&period, &symbol, "".to_string()).await,
                            stats,
                        );
                    }
                    //  else {
                    //     println!("USED CACHE FOR CASH - {}!", &symbol);
                    // }
                }
                TimePeriod::Quarter(v) => {
                    if (self.quarter_cash.0.len() < v as usize
                        && v > self.quarter_cash.1.last_pull_length as u8)
                        || should_update
                    {
                        self.quarter_cash = (
                            api::<CashFlowStatement>(&period, &symbol, "".to_string()).await,
                            stats,
                        );
                    } 
                    // else {
                    //     println!("USED CACHE FOR CASH - {} (QTR)!", &symbol);
                    // }
                }
                _ => {}
            }
        }
    }
}
