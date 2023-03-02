use crate::helper_structs::TimePeriod;
use crate::utils;

use core::fmt::Debug;
use serde::de::DeserializeOwned;

use std::time::Duration;
use tokio::time;

static mut COUNTER: u64 = 0;

async fn api_handler<T>(
    symbol: &String,
    period: &TimePeriod,
    q: String,
) -> Result<Vec<T>, reqwest::Error>
where
    T: DeserializeOwned + Debug + 'static,
{
    let limit: String;
    let period_type: String;
    let key = utils::api_key();

    let mut end_point = utils::type_to_api_format::<T>();

    if end_point == "available-traded".to_string() {
        end_point += "/list";
    }

    match period {
        TimePeriod::Annual(years) => {
            limit = years.to_string();
            period_type = String::from("annual");
        }
        TimePeriod::Quarter(quarters) => {
            limit = quarters.to_string();
            period_type = String::from("quarter");
        }
        _ => {
            limit = String::from("");
            period_type = String::from("");
        }
    }

    let url = format!(
        "https://financialmodelingprep.com/api/v3/{}/{}?apikey={}&limit={}&period={}&{}",
        end_point, &symbol, key, limit, period_type, q
    );

    let mut interval = time::interval(Duration::from_secs_f32(0.2));

    unsafe {
        println!("{}   {}", COUNTER, url);
        COUNTER += 1;
        interval.tick().await;
        interval.tick().await;
    }

    let resp: Vec<T> = reqwest::get(url).await?.json::<Vec<T>>().await?;

    Ok(resp)
}

pub async fn api<T>(period: &TimePeriod, symbol: &String, q: String) -> Vec<T>
where
    T: DeserializeOwned + Debug + 'static,
{
    let mut statements = Vec::new();
    let result = api_handler::<T>(&symbol, &period, q).await;

    match result {
        Ok(v) => {
            statements = v;
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    statements
}
