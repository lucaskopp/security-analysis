use crate::{
    helper_structs::{
        BalanceSheetStatement, CashFlowStatement, IncomeStatement, KeyMetrics, KeyMetricsTTM,
        NeededData, Profile, Ratios, RatiosTTM, TimePeriod,
    },
    metrics::Metrics,
    other::Other,
    statements::Statements,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stock {
    pub cache_index: Option<usize>,
    pub ticker: String,
    pub statements: Statements,
    pub metrics: Metrics,
    pub other: Other,
}

impl Stock {
    pub fn set_cache_index(&mut self, index: Option<usize>) {
        self.cache_index = index;
    }

    pub async fn income(&mut self, period: TimePeriod) {
        self.statements
            .fetch::<IncomeStatement>(period, &self.ticker)
            .await;
    }

    pub async fn balance(&mut self, period: TimePeriod) {
        self.statements
            .fetch::<BalanceSheetStatement>(period, &self.ticker)
            .await;
    }

    pub async fn cash(&mut self, period: TimePeriod) {
        self.statements
            .fetch::<CashFlowStatement>(period, &self.ticker)
            .await;
    }

    pub async fn ratios(&mut self, period: TimePeriod) {
        self.metrics.fetch::<Ratios>(period, &self.ticker).await;
    }

    pub async fn ratios_ttm(&mut self) {
        self.metrics
            .fetch::<RatiosTTM>(TimePeriod::TTM(), &self.ticker)
            .await;
    }

    pub async fn key_metrics(&mut self, period: TimePeriod) {
        self.metrics.fetch::<KeyMetrics>(period, &self.ticker).await;
    }

    pub async fn key_metrics_ttm(&mut self) {
        self.metrics
            .fetch::<KeyMetricsTTM>(TimePeriod::TTM(), &self.ticker)
            .await;
    }

    pub async fn profile(&mut self) {
        self.other.fetch::<Profile>(&self.ticker).await;
    }

    pub async fn get_all(&mut self) {
        self.income(TimePeriod::Annual(10)).await;
        self.income(TimePeriod::Quarter(8)).await;
        self.balance(TimePeriod::Annual(10)).await;
        self.balance(TimePeriod::Quarter(8)).await;
        self.cash(TimePeriod::Annual(10)).await;
        self.cash(TimePeriod::Quarter(8)).await;
        self.ratios(TimePeriod::Annual(10)).await;
        self.ratios_ttm().await;
        self.key_metrics(TimePeriod::Annual(10)).await;
        self.key_metrics_ttm().await;
        self.profile().await;
    }

    pub async fn get_needed_data(&mut self, needed: NeededData) -> &mut Self {
        if needed.income.0 {
            self.income(needed.income.1).await;
        }

        if needed.balance.0 {
            self.balance(needed.balance.1).await;
        }

        if needed.ratios.0 {
            self.ratios(needed.ratios.1).await;
        }

        if needed.key_metrics.0 {
            self.key_metrics(needed.key_metrics.1).await;
        }

        if needed.key_metrics_ttm.0 {
            self.key_metrics_ttm().await;
        }

        self
    }
}
