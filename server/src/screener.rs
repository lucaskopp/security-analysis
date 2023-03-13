use std::vec;

use crate::helper_functions::api;
use crate::helper_structs::{AvailableTraded, TimePeriod};
use crate::{cache, CACHE};

#[derive(Debug)]
pub struct Screener {
    pub stocks_to_screen: Vec<usize>,
}

impl Screener {
    pub fn new() -> Self {
        Self {
            stocks_to_screen: Vec::new(),
        }
    }

    pub async fn init_screen(&mut self) {
        let symbols =
            api::<AvailableTraded>(&TimePeriod::NA(), &"".to_string(), "".to_string()).await;

        self.stocks_to_screen = Screener::symbols_to_stocks(symbols).await;
    }

    async fn symbols_to_stocks(symbols: Vec<AvailableTraded>) -> Vec<usize> {
        let mut stocks = vec![];
        for stock in symbols {
            if stock.type_ == "stock"
                && (stock.exchange_short_name == "NYSE" || stock.exchange_short_name == "NASDAQ")
            {
                stocks.push(
                    cache::get_or_add_stock(stock.symbol)
                        .await
                        .cache_index
                        .unwrap(),
                );
            }
        }

        stocks
    }

    pub async fn buffetology_screener(&mut self) -> Vec<usize> {
        let mut passed = vec![];
        // self.stocks_to_screen.len()
        for i in 0..self.stocks_to_screen.len() {
            let stock = self.stocks_to_screen[i];
            if self.is_buffetology_stock(stock).await {
                passed.push(stock.to_owned());
            }
        }

        passed
    }

    pub async fn index_everything(&mut self) {
        // self.stocks_to_screen.len()
        for i in 0..self.stocks_to_screen.len() {
            let mut cache = CACHE.lock().await;
            let stock = cache.get_mut(self.stocks_to_screen[i]).unwrap();
            let current_percent =
                ((i as f64 + 1.0) / (self.stocks_to_screen.len() as f64)) * 100.0;

            stock.get_all().await;

            println!("{}%", current_percent);
        }
    }

    async fn is_buffetology_stock(&mut self, stock_index: usize) -> bool {
        // stock
        //     .get_needed_data(NeededData {
        //         income: (true, TimePeriod::Annual(10)),
        //         balance: (true, TimePeriod::Quarter(1)),
        //         ratios: (true, TimePeriod::Annual(10)),
        //         key_metrics: (true, TimePeriod::Annual(10)),
        //         key_metrics_ttm: (true, TimePeriod::TTM()),
        //         income_qtr: (false, TimePeriod::NA()),
        //     })
        //     .await;

        let mut cache = CACHE.lock().await;
        let stock = cache.get_mut(stock_index).unwrap();

        stock.income(TimePeriod::Annual(10)).await;
        let income_statements = stock.statements.annual_income.clone();

        if income_statements.0.len() != 10 {
            return false;
        }

        if income_statements.0[0].eps < income_statements.0[4].eps
            || income_statements.0[4].eps < income_statements.0[9].eps
        {
            return false;
        }

        for i in 0..10 {
            if income_statements.0[i].eps < Some(0.0) {
                return false;
            }
        }

        stock.key_metrics(TimePeriod::Annual(10)).await;
        let key_metrics = &stock.metrics.annual_key_metrics;

        if key_metrics.0.len() != 10 {
            return false;
        }

        let mut roic_mean = 0.0;

        for i in 0..10 {
            if let Some(roic) = key_metrics.0[i].roic {
                roic_mean += roic;
            }
        }

        roic_mean /= 10.0;

        if roic_mean < 0.12 {
            return false;
        }

        stock.ratios(TimePeriod::Annual(10)).await;
        let ratios = &stock.metrics.annual_ratios;

        if ratios.0.len() != 10 {
            return false;
        }

        let mut roe_mean = 0.0;
        for i in 0..10 {
            if let Some(roe) = ratios.0[i].return_on_equity {
                roe_mean += roe;
            }
        }

        roe_mean /= 10.0;

        if roe_mean < 0.15 {
            return false;
        }

        stock.key_metrics_ttm().await;
        let key_metrics = &stock.metrics.ttm_key_metrics;

        if key_metrics.0.len() == 0 {
            return false;
        }

        let earnings_yield = stock.metrics.ttm_key_metrics.0[0].earnings_yield_TTM;

        if earnings_yield < Some(0.03) {
            return false;
        }

        stock.balance(TimePeriod::Quarter(1)).await;
        let balance_sheet = &stock.statements.quarter_balance;

        if let Some(long_term_debt) = balance_sheet.0[0].long_term_debt {
            if let Some(net_income) = income_statements.0[0].net_income {
                if long_term_debt > net_income * 5.0 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        // println!("{} YOU MADE IT!!!", stock.ticker);

        true
    }
}
