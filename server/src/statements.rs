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
    pub ttm_income: (Vec<IncomeStatement>, FetchStats),
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
            ttm_income: (Vec::new(), FetchStats::new(0)),
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
                TimePeriod::TTM() => {
                    if (self.ttm_income.0.len() == 0
                        && 1 > self.ttm_income.1.last_pull_length as u8)
                        || should_update
                    {
                        self.fetch::<IncomeStatement>(TimePeriod::Quarter(10), symbol).await;
                        let most_recent = &self.quarter_income.0[0];

                        let mut ttm_income = IncomeStatement {
                            date: most_recent.date.clone(),
                            cost_and_expenses: most_recent.cost_and_expenses,
                            cost_of_revenue: most_recent.cost_of_revenue,
                            depreciation_and_amortization: most_recent.depreciation_and_amortization,
                            ebitda: most_recent.ebitda,
                            ebitdaratio: None,
                            eps: most_recent.eps,
                            epsdiluted: most_recent.epsdiluted,
                            general_and_administrative_expenses: most_recent.general_and_administrative_expenses,
                            gross_profit: most_recent.gross_profit,
                            revenue: most_recent.revenue,
                            gross_profit_ratio: None,
                            income_before_tax: most_recent.income_before_tax, 
                            income_before_tax_ratio: None,
                            income_tax_expense: most_recent.income_tax_expense,
                            interest_expense: most_recent.interest_expense, 
                            interest_income: most_recent.interest_income,
                            net_income: most_recent.net_income,
                            net_income_ratio: None,
                            operating_expenses: most_recent.operating_expenses,
                            operating_income: most_recent.operating_income,
                            operating_income_ratio: None,
                            other_expenses: most_recent.other_expenses,
                            research_and_development_expenses: most_recent.research_and_development_expenses,
                            selling_and_marketing_expenses: most_recent.selling_and_marketing_expenses,
                            selling_general_and_administrative_expenses: most_recent.selling_general_and_administrative_expenses,
                            total_other_income_expenses_net: most_recent.total_other_income_expenses_net,
                            weighted_average_shs_out: most_recent.weighted_average_shs_out,
                            weighted_average_shs_out_dil: most_recent.weighted_average_shs_out_dil,

                        };

                        for i in [1..4] {
                            ttm_income.cost_and_expenses = &self.quarter_income.0[i];
                        }
                    }
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
