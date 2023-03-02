use core::fmt::Debug;
use serde::de::DeserializeOwned;
use std::any::TypeId;

use crate::helper_functions::api;
use crate::helper_structs::{FetchStats, KeyMetrics, KeyMetricsTTM, Ratios, RatiosTTM, TimePeriod};
use crate::utils::{needs_update_based_on_time, update_pull_stats};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metrics {
    pub annual_ratios: (Vec<Ratios>, FetchStats),
    pub quarter_ratios: (Vec<Ratios>, FetchStats),
    pub ttm_ratios: (Vec<RatiosTTM>, FetchStats),
    pub annual_key_metrics: (Vec<KeyMetrics>, FetchStats),
    pub quarter_key_metrics: (Vec<KeyMetrics>, FetchStats),
    pub ttm_key_metrics: (Vec<KeyMetricsTTM>, FetchStats),
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            annual_ratios: (Vec::new(), FetchStats::new(0)),
            quarter_ratios: (Vec::new(), FetchStats::new(0)),
            ttm_ratios: (Vec::new(), FetchStats::new(0)),
            annual_key_metrics: (Vec::new(), FetchStats::new(0)),
            quarter_key_metrics: (Vec::new(), FetchStats::new(0)),
            ttm_key_metrics: (Vec::new(), FetchStats::new(0)),
        }
    }

    pub async fn fetch<T>(&mut self, period: TimePeriod, symbol: &String)
    where
        T: DeserializeOwned + Debug + 'static,
    {
        let should_update = needs_update_based_on_time(self, &period);
        let stats = update_pull_stats(&period);

        if TypeId::of::<T>() == TypeId::of::<Ratios>() {
            match period {
                TimePeriod::Annual(v) => {
                    if (self.annual_ratios.0.len() < v as usize
                        && v > self.annual_ratios.1.last_pull_length as u8)
                        || should_update
                    {
                        self.annual_ratios =
                            (api::<Ratios>(&period, &symbol, "".to_string()).await, stats);
                    }
                    // else {
                    //     println!("USED CACHE FOR RATIOS - {}!", &symbol);
                    // }
                }
                TimePeriod::Quarter(v) => {
                    if (self.quarter_ratios.0.len() < v as usize
                        && v > self.quarter_ratios.1.last_pull_length as u8)
                        || should_update
                    {
                        self.quarter_ratios =
                            (api::<Ratios>(&period, &symbol, "".to_string()).await, stats);
                    }
                    // else {
                    //     println!("USED CACHE FOR RATIOS (QTR) - {}!", &symbol);
                    // }
                }
                _ => {}
            }
        } else if TypeId::of::<T>() == TypeId::of::<KeyMetrics>() {
            match period {
                TimePeriod::Annual(v) => {
                    if (self.annual_key_metrics.0.len() < v as usize
                        && v > self.annual_key_metrics.1.last_pull_length as u8)
                        || should_update
                    {
                        self.annual_key_metrics = (
                            api::<KeyMetrics>(&period, &symbol, "".to_string()).await,
                            stats,
                        );
                    }
                    // else {
                    //     println!("USED CACHE FOR KEY_METRICS - {}!", &symbol);
                    // }
                }
                TimePeriod::Quarter(v) => {
                    if (self.quarter_key_metrics.0.len() < v as usize
                        && v > self.quarter_key_metrics.1.last_pull_length as u8)
                        || should_update
                    {
                        self.quarter_key_metrics = (
                            api::<KeyMetrics>(&period, &symbol, "".to_string()).await,
                            stats,
                        );
                    }
                    // else {
                    //     println!("USED CACHE FOR KEY_METRICS (QTR) - {}!", &symbol);
                    // }
                }
                _ => {}
            }
        } else if TypeId::of::<T>() == TypeId::of::<RatiosTTM>() {
            if (self.ttm_ratios.0.len() == 0 && 1 > self.ttm_ratios.1.last_pull_length as u8)
                || should_update
            {
                self.ttm_ratios = (
                    api::<RatiosTTM>(&period, &symbol, "".to_string()).await,
                    stats,
                );
            }
            // else {
            //     println!("USED CACHE RATIOS_TTM - {}!", &symbol);
            // }
        } else if TypeId::of::<T>() == TypeId::of::<KeyMetricsTTM>() {
            if (self.ttm_key_metrics.0.len() == 0
                && 1 > self.ttm_key_metrics.1.last_pull_length as u8)
                || should_update
            {
                self.ttm_key_metrics = (
                    api::<KeyMetricsTTM>(&period, &symbol, "".to_string()).await,
                    stats,
                );
            }
            // else {
            //     println!("USED CACHE FOR KEY_METRICS_TTM - {}!", &symbol);
            // }
        }
    }
}
