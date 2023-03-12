use crate::{metrics::Metrics, statements::Statements, stock::Stock, CACHE, other::Other};
use tokio::sync::{MutexGuard, MappedMutexGuard};
use std::{fs, ops::Deref};

// Make struct and pass it from main into the needed areas

pub async fn get_or_add_stock(symbol: String) -> MappedMutexGuard<'static, Stock> {
    let stock_exists = stock_index_in_cache(symbol.to_owned());

    match stock_exists.await {
        Some(v) => MutexGuard::map(CACHE.lock().await, |d| d.get_mut(v).unwrap()),
        None => add_stock(Stock {
            cache_index: None,
            ticker: symbol,
            statements: Statements::new(),
            metrics: Metrics::new(),
            other: Other::new(),
        }).await,
    }
}

pub async fn add_stock(mut stock: Stock) -> MappedMutexGuard<'static, Stock> {
    let length = CACHE.lock().await.len();

    stock.set_cache_index(Some(length));

    CACHE.lock().await.push(stock);

    return MutexGuard::map(CACHE.lock().await, |d| d.get_mut(length).unwrap());
}

pub async fn stock_index_in_cache(ticker: String) -> Option<usize> {
    CACHE.lock().await.iter().position(|stock| stock.ticker == ticker)
}

pub async fn save() {
    let struct_as_json = serde_json::to_string(&CACHE.lock().await.deref()).unwrap();

    let write_to_file = fs::write("cache.json", struct_as_json);

    match write_to_file {
        Ok(_v) => {
            println!("WROTE TO FILE");
        }
        Err(e) => {
            println!("Could not write to file: {}", e);
        }
    }
}

pub fn state_from_json() -> Vec<Stock> {
    let cache_location = fs::read_to_string("cache.json");
    let mut cache_string = String::from(r#"[{}]"#);

    match cache_location {
        Ok(v) => {
            cache_string = v;
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    let string_to_struct = serde_json::from_str(&cache_string);

    match string_to_struct {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            Vec::new()
        }
    }
}
