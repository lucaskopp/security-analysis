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
        let mut stats = update_pull_stats(&period);

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
                        let ttm_stats = stats.clone();

                        if (self.quarter_income.0.len() < 8 as usize
                            && 8 > self.quarter_income.1.last_pull_length as u8)
                            || should_update
                        {
                            stats = update_pull_stats(&TimePeriod::Quarter(8));

                            self.quarter_income = (
                                api::<IncomeStatement>(&period, &symbol, "".to_string()).await,
                                stats,
                            );
                        }

                        let mut ttm_income = IncomeStatement {
                            date: String::from("TTM"),
                            cost_and_expenses: None,
                            cost_of_revenue: None,
                            depreciation_and_amortization: None,
                            ebitda: None,
                            ebitdaratio: None,
                            eps: None,
                            epsdiluted: None,
                            general_and_administrative_expenses: None,
                            gross_profit: None,
                            revenue: None,
                            gross_profit_ratio: None,
                            income_before_tax: None,
                            income_before_tax_ratio: None,
                            income_tax_expense: None,
                            interest_expense: None,
                            interest_income: None,
                            net_income: None,
                            net_income_ratio: None,
                            operating_expenses: None,
                            operating_income: None,
                            operating_income_ratio: None,
                            other_expenses: None,
                            research_and_development_expenses: None,
                            selling_and_marketing_expenses: None,
                            selling_general_and_administrative_expenses: None,
                            total_other_income_expenses_net: None,
                            weighted_average_shs_out: None,
                            weighted_average_shs_out_dil: None,
                        };

                        for i in 0..4 {
                            let statement = self.quarter_income.0.get(i);
                            match statement {
                                Some(statement) => {
                                    ttm_income.cost_and_expenses = Some(
                                        statement.cost_and_expenses.unwrap_or_default()
                                            + ttm_income.cost_and_expenses.unwrap_or_default(),
                                    );

                                    ttm_income.cost_of_revenue = Some(
                                        statement.cost_of_revenue.unwrap_or_default()
                                            + ttm_income.cost_of_revenue.unwrap_or_default(),
                                    );

                                    ttm_income.depreciation_and_amortization = Some(
                                        statement.depreciation_and_amortization.unwrap_or_default()
                                            + ttm_income.depreciation_and_amortization.unwrap_or_default(),
                                    );

                                    ttm_income.ebitda = Some(
                                        statement.ebitda.unwrap_or_default()
                                            + ttm_income.ebitda.unwrap_or_default(),
                                    );

                                    ttm_income.eps = Some(
                                        statement.eps.unwrap_or_default()
                                            + ttm_income.eps.unwrap_or_default(),
                                    );

                                    ttm_income.epsdiluted = Some(
                                        statement.epsdiluted.unwrap_or_default()
                                            + ttm_income.epsdiluted.unwrap_or_default(),
                                    );

                                    ttm_income.general_and_administrative_expenses = Some(
                                        statement.general_and_administrative_expenses.unwrap_or_default()
                                            + ttm_income.general_and_administrative_expenses.unwrap_or_default(),
                                    );

                                    ttm_income.gross_profit = Some(
                                        statement.gross_profit.unwrap_or_default()
                                            + ttm_income.gross_profit.unwrap_or_default(),
                                    );

                                    ttm_income.revenue = Some(
                                        statement.revenue.unwrap_or_default()
                                            + ttm_income.revenue.unwrap_or_default(),
                                    );

                                    ttm_income.income_before_tax = Some(
                                        statement.income_before_tax.unwrap_or_default()
                                            + ttm_income.income_before_tax.unwrap_or_default(),
                                    );

                                    ttm_income.income_tax_expense = Some(
                                        statement.income_tax_expense.unwrap_or_default()
                                            + ttm_income.income_tax_expense.unwrap_or_default(),
                                    );

                                    ttm_income.interest_expense = Some(
                                        statement.interest_expense.unwrap_or_default()
                                            + ttm_income.interest_expense.unwrap_or_default(),
                                    );

                                    ttm_income.interest_income = Some(
                                        statement.interest_income.unwrap_or_default()
                                            + ttm_income.interest_income.unwrap_or_default(),
                                    );

                                    ttm_income.net_income = Some(
                                        statement.net_income.unwrap_or_default()
                                            + ttm_income.net_income.unwrap_or_default(),
                                    );

                                    ttm_income.operating_expenses = Some(
                                        statement.operating_expenses.unwrap_or_default()
                                            + ttm_income.operating_expenses.unwrap_or_default(),
                                    );

                                    ttm_income.operating_income = Some(
                                        statement.operating_income.unwrap_or_default()
                                            + ttm_income.operating_income.unwrap_or_default(),
                                    );

                                    ttm_income.other_expenses = Some(
                                        statement.other_expenses.unwrap_or_default()
                                            + ttm_income.other_expenses.unwrap_or_default(),
                                    );

                                    ttm_income.research_and_development_expenses = Some(
                                        statement.research_and_development_expenses.unwrap_or_default()
                                            + ttm_income.research_and_development_expenses.unwrap_or_default(),
                                    );

                                    ttm_income.selling_and_marketing_expenses = Some(
                                        statement.selling_and_marketing_expenses.unwrap_or_default()
                                            + ttm_income.selling_and_marketing_expenses.unwrap_or_default(),
                                    );

                                    ttm_income.selling_general_and_administrative_expenses = Some(
                                        statement.selling_general_and_administrative_expenses.unwrap_or_default()
                                            + ttm_income.selling_general_and_administrative_expenses.unwrap_or_default(),
                                    );

                                    ttm_income.total_other_income_expenses_net = Some(
                                        statement.total_other_income_expenses_net.unwrap_or_default()
                                            + ttm_income.total_other_income_expenses_net.unwrap_or_default(),
                                    );
                                }
                                None => {}
                            }
                        }

                        self.ttm_income = (vec![ttm_income], ttm_stats);
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
