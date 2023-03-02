use std::env;

use std::any::type_name;

use chrono::{NaiveDate, Utc};
use convert_case::{Case, Casing};

use crate::helper_structs::{FetchStats, StockInfo, TimePeriod};
use crate::statements::Statements;

pub fn api_key() -> String {
    let mut key = String::from("");

    match env::var("FINANCIAL_API") {
        Ok(v) => {
            key = v.to_string();
        }
        Err(_e) => (),
    }

    key
}

pub fn type_to_api_format<T>() -> String {
    let mut type_string = type_name::<T>().to_string();
    let mut colon_exist = Some(0);

    while colon_exist != Option::None {
        colon_exist = type_string.find(":");

        match colon_exist {
            Some(v) => {
                type_string = type_string[v + 2..].to_string();
            }
            None => {
                return type_string.to_case(Case::Kebab);
            }
        }
    }

    type_string.to_case(Case::Kebab)
}

pub fn needs_update_based_on_time(statements: &dyn StockInfo, period: &TimePeriod) -> bool {
    match period {
        TimePeriod::Annual(_v) => {
            if statements.length_of_annual_statement() != 0 {
                let today = Utc::now();
                let annual_time_extended =
                    NaiveDate::parse_from_str(statements.recent_annual_date(), "%Y-%m-%d")
                        .unwrap()
                        .checked_add_days(chrono::Days::new(355))
                        .unwrap();

                let annual_diff = (annual_time_extended - today.date_naive()).num_days();

                let last_pull_time = statements.last_pull_time_annual();
                let time_since_last_pull = (today.date_naive() - last_pull_time).num_days();

                return annual_diff < 0 && time_since_last_pull > 0;
            }
        }
        TimePeriod::Quarter(_) | TimePeriod::TTM() => {
            if statements.length_of_quarter_statement() != 0 {
                let today = Utc::now();
                let quarter_time_extended =
                    NaiveDate::parse_from_str(statements.recent_quarter_date(), "%Y-%m-%d")
                        .unwrap()
                        .checked_add_days(chrono::Days::new(80))
                        .unwrap();

                let quarter_diff = (quarter_time_extended - today.date_naive()).num_days();

                let last_pull_time = statements.last_pull_time_quarter();
                let time_since_last_pull = (today.date_naive() - last_pull_time).num_days();

                return quarter_diff < 0 && time_since_last_pull > 0;
            }
        }
        _ => {}
    }

    false
}

pub fn update_pull_stats(period: &TimePeriod) -> FetchStats {
    let mut stats;
    match period {
        TimePeriod::Annual(v) | TimePeriod::Quarter(v) => {
            stats = FetchStats::new(*v as usize);
            stats.last_pull_time = Some(Utc::now().date_naive());
            stats
        }
        _ => {
            stats = FetchStats::new(1);
            stats.last_pull_time = Some(Utc::now().date_naive());
            stats
        }
    }
}
