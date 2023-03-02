use crate::{
    helper_structs::{
        BalanceSheetStatement, IncomeStatement, KeyMetrics, KeyMetricsTTM, NeededData, Ratios,
        TimePeriod,
    },
    metrics::Metrics,
    statements::Statements,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stock {
    pub cache_index: Option<usize>,
    pub ticker: String,
    pub statements: Statements,
    pub metrics: Metrics,
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

    pub async fn ratios(&mut self, period: TimePeriod) {
        self.metrics.fetch::<Ratios>(period, &self.ticker).await;
    }

    pub async fn key_metrics(&mut self, period: TimePeriod) {
        self.metrics.fetch::<KeyMetrics>(period, &self.ticker).await;
    }

    pub async fn key_metrics_ttm(&mut self, period: TimePeriod) {
        self.metrics
            .fetch::<KeyMetricsTTM>(period, &self.ticker)
            .await;
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
            self.key_metrics_ttm(needed.key_metrics_ttm.1).await;
        }

        self
    }
}
