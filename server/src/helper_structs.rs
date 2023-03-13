use axum::Json;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::stock::Stock;
use crate::{metrics::Metrics, statements::Statements};

// Use struct instead of tuple for better readability
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TimePeriod {
    Annual(u8),
    Quarter(u8),
    TTM(),
    NA(),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvailableTraded {
    pub symbol: String,
    pub exchange_short_name: String,
    pub type_: String,
}

#[derive(Debug)]
pub struct ResponseCache {
    pub endpoint: String,
    pub data: Json<Vec<Stock>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncomeStatement {
    pub date: String,
    pub revenue: Option<f64>,
    pub cost_of_revenue: Option<f64>,
    pub gross_profit: Option<f64>,
    pub gross_profit_ratio: Option<f64>,
    pub research_and_development_expenses: Option<f64>,
    pub general_and_administrative_expenses: Option<f64>,
    pub selling_and_marketing_expenses: Option<f64>,
    pub selling_general_and_administrative_expenses: Option<f64>,
    pub other_expenses: Option<f64>,
    pub operating_expenses: Option<f64>,
    pub cost_and_expenses: Option<f64>,
    pub interest_income: Option<f64>,
    pub interest_expense: Option<f64>,
    pub depreciation_and_amortization: Option<f64>,
    pub ebitda: Option<f64>,
    pub ebitdaratio: Option<f64>,
    pub operating_income: Option<f64>,
    pub operating_income_ratio: Option<f64>,
    pub total_other_income_expenses_net: Option<f64>,
    pub income_before_tax: Option<f64>,
    pub income_before_tax_ratio: Option<f64>,
    pub income_tax_expense: Option<f64>,
    pub net_income: Option<f64>,
    pub net_income_ratio: Option<f64>,
    pub eps: Option<f64>,
    pub epsdiluted: Option<f64>,
    pub weighted_average_shs_out: Option<f64>,
    pub weighted_average_shs_out_dil: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheetStatement {
    pub date: String,
    pub cash_and_cash_equivalents: Option<f64>,
    pub short_term_investments: Option<f64>,
    pub cash_and_short_term_investments: Option<f64>,
    pub net_receivables: Option<f64>,
    pub inventory: Option<f64>,
    pub other_current_assets: Option<f64>,
    pub total_current_assets: Option<f64>,
    pub property_plant_equipment_net: Option<f64>,
    pub goodwill: Option<f64>,
    pub intangible_assets: Option<f64>,
    pub goodwill_and_intangible_assets: Option<f64>,
    pub long_term_investments: Option<f64>,
    pub tax_assets: Option<f64>,
    pub other_non_current_assets: Option<f64>,
    pub total_non_current_assets: Option<f64>,
    pub other_assets: Option<f64>,
    pub total_assets: Option<f64>,
    pub account_payables: Option<f64>,
    pub deferred_revenue: Option<f64>,
    pub other_current_liabilities: Option<f64>,
    pub total_current_liabilities: Option<f64>,
    pub long_term_debt: Option<f64>,
    pub deferred_revenue_non_current: Option<f64>,
    pub deferred_tax_liabilities_non_current: Option<f64>,
    pub other_non_current_liabilities: Option<f64>,
    pub total_non_current_liabilities: Option<f64>,
    pub other_liabilities: Option<f64>,
    pub capital_lease_obligations: Option<f64>,
    pub total_liabilities: Option<f64>,
    pub preferred_stock: Option<f64>,
    pub common_stock: Option<f64>,
    pub retained_earnings: Option<f64>,
    pub accumulated_other_comprehensive_income_loss: Option<f64>,
    pub othertotal_stockholders_equity: Option<f64>,
    pub total_stockholders_equity: Option<f64>,
    pub total_equity: Option<f64>,
    pub total_liabilities_and_stockholders_equity: Option<f64>,
    pub minority_interest: Option<f64>,
    pub total_liabilities_and_total_equity: Option<f64>,
    pub total_investments: Option<f64>,
    pub total_debt: Option<f64>,
    pub net_debt: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CashFlowStatement {
    pub date: String,
    pub net_income: Option<f64>,
    pub depreciation_and_amortization: Option<f64>,
    pub deferred_income_tax: Option<f64>,
    pub stock_based_compensation: Option<f64>,
    pub change_in_working_capital: Option<f64>,
    pub accounts_receivables: Option<f64>,
    pub inventory: Option<f64>,
    pub accounts_payables: Option<f64>,
    pub other_working_capital: Option<f64>,
    pub other_non_cash_items: Option<f64>,
    pub net_cash_provided_by_operating_activities: Option<f64>,
    pub investments_in_property_plant_and_equipment: Option<f64>,
    pub acquisitions_net: Option<f64>,
    pub purchases_of_investments: Option<f64>,
    pub sales_maturities_of_investments: Option<f64>,
    pub other_investing_activites: Option<f64>,
    pub net_cash_used_for_investing_activites: Option<f64>,
    pub debt_repayment: Option<f64>,
    pub common_stock_issued: Option<f64>,
    pub common_stock_repurchased: Option<f64>,
    pub dividends_paid: Option<f64>,
    pub other_financing_activites: Option<f64>,
    pub net_cash_used_provided_by_financing_activities: Option<f64>,
    pub effect_of_forex_changes_on_cash: Option<f64>,
    pub net_change_in_cash: Option<f64>,
    pub cash_at_end_of_period: Option<f64>,
    pub cash_at_beginning_of_period: Option<f64>,
    pub operating_cash_flow: Option<f64>,
    pub capital_expenditure: Option<f64>,
    pub free_cash_flow: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ratios {
    pub date: String,
    pub current_ratio: Option<f64>,
    pub quick_ratio: Option<f64>,
    pub cash_ratio: Option<f64>,
    pub days_of_sales_outstanding: Option<f64>,
    pub days_of_inventory_outstanding: Option<f64>,
    pub operating_cycle: Option<f64>,
    pub days_of_payables_outstanding: Option<f64>,
    pub cash_conversion_cycle: Option<f64>,
    pub gross_profit_margin: Option<f64>,
    pub operating_profit_margin: Option<f64>,
    pub pretax_profit_margin: Option<f64>,
    pub net_profit_margin: Option<f64>,
    pub effective_tax_rate: Option<f64>,
    pub return_on_assets: Option<f64>,
    pub return_on_equity: Option<f64>,
    pub return_on_capital_employed: Option<f64>,
    pub net_income_per_EBT: Option<f64>,
    pub ebit_per_revenue: Option<f64>,
    pub debt_ratio: Option<f64>,
    pub debt_equity_ratio: Option<f64>,
    pub long_term_debt_to_capitalization: Option<f64>,
    pub total_debt_to_capitalization: Option<f64>,
    pub interest_coverage: Option<f64>,
    pub cash_flow_to_debt_ratio: Option<f64>,
    pub company_equity_multiplier: Option<f64>,
    pub receivables_turnover: Option<f64>,
    pub payables_turnover: Option<f64>,
    pub inventory_turnover: Option<f64>,
    pub fixed_asset_turnover: Option<f64>,
    pub asset_turnover: Option<f64>,
    pub operating_cash_flow_per_share: Option<f64>,
    pub free_cash_flow_per_share: Option<f64>,
    pub cash_flow_coverage_ratios: Option<f64>,
    pub short_term_coverage_ratios: Option<f64>,
    pub capital_expenditure_coverage_ratio: Option<f64>,
    pub dividend_paid_and_capex_coverage_ratio: Option<f64>,
    pub dividend_payout_ratio: Option<f64>,
    pub price_book_value_ratio: Option<f64>,
    pub price_to_book_ratio: Option<f64>,
    pub price_to_sales_ratio: Option<f64>,
    pub price_earnings_ratio: Option<f64>,
    pub price_to_free_cash_flows_ratio: Option<f64>,
    pub price_to_operating_cash_flows_ratio: Option<f64>,
    pub price_cash_flow_ratio: Option<f64>,
    pub price_earnings_to_growth_ratio: Option<f64>,
    pub price_sales_ratio: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub enterprise_value_multiple: Option<f64>,
    pub priceFairValue: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RatiosTTM {
    pub dividend_yiel_TTM: Option<f64>,
    pub dividend_yiel_percentage_TTM: Option<f64>,
    pub pe_ratio_TTM: Option<f64>,
    pub peg_ratio_TTM: Option<f64>,
    pub payout_ratio_TTM: Option<f64>,
    pub current_ratio_TTM: Option<f64>,
    pub quick_ratio_TTM: Option<f64>,
    pub cash_ratio_TTM: Option<f64>,
    pub days_of_sales_outstanding_TTM: Option<f64>,
    pub days_of_inventory_outstanding_TTM: Option<f64>,
    pub operating_cycle_TTM: Option<f64>,
    pub days_of_payables_outstanding_TTM: Option<f64>,
    pub cash_conversion_cycle_TTM: Option<f64>,
    pub gross_profit_margin_TTM: Option<f64>,
    pub operating_profit_margin_TTM: Option<f64>,
    pub pretax_profit_margin_TTM: Option<f64>,
    pub net_profit_margin_TTM: Option<f64>,
    pub effective_tax_rate_TTM: Option<f64>,
    pub return_on_assets_TTM: Option<f64>,
    pub return_on_equity_TTM: Option<f64>,
    pub return_on_capital_employed_TTM: Option<f64>,
    pub net_income_per_EBTTTM: Option<f64>,
    pub ebt_per_ebit_TTM: Option<f64>,
    pub ebit_per_revenue_TTM: Option<f64>,
    pub debt_ratio_TTM: Option<f64>,
    pub debt_equity_ratio_TTM: Option<f64>,
    pub long_term_debt_to_capitalization_TTM: Option<f64>,
    pub total_debt_to_capitalization_TTM: Option<f64>,
    pub interest_coverage_TTM: Option<f64>,
    pub cash_flow_to_debt_ratio_TTM: Option<f64>,
    pub company_equity_multiplier_TTM: Option<f64>,
    pub receivables_turnover_TTM: Option<f64>,
    pub payables_turnover_TTM: Option<f64>,
    pub inventory_turnover_TTM: Option<f64>,
    pub fixed_asset_turnover_TTM: Option<f64>,
    pub asset_turnover_TTM: Option<f64>,
    pub operating_cash_flow_per_share_TTM: Option<f64>,
    pub free_cash_flow_per_share_TTM: Option<f64>,
    pub cash_per_share_TTM: Option<f64>,
    pub operating_cash_flow_sales_ratio_TTM: Option<f64>,
    pub free_cash_flow_operating_cash_flow_ratio_TTM: Option<f64>,
    pub cash_flow_coverage_ratios_TTM: Option<f64>,
    pub short_term_coverage_ratios_TTM: Option<f64>,
    pub capital_expenditure_coverage_ratio_TTM: Option<f64>,
    pub dividend_paid_and_capex_coverage_ratio_TTM: Option<f64>,
    pub price_book_value_ratio_TTM: Option<f64>,
    pub price_to_book_ratio_TTM: Option<f64>,
    pub price_to_sales_ratio_TTM: Option<f64>,
    pub price_earnings_ratio_TTM: Option<f64>,
    pub price_to_free_cash_flows_ratio_TTM: Option<f64>,
    pub price_to_operating_cash_flows_ratio_TTM: Option<f64>,
    pub price_cash_flow_ratio_TTM: Option<f64>,
    pub price_earnings_to_growth_ratio_TTM: Option<f64>,
    pub price_sales_ratio_TTM: Option<f64>,
    pub dividend_yield_TTM: Option<f64>,
    pub enterprise_value_multiple_TTM: Option<f64>,
    pub price_fair_value_TTM: Option<f64>,
    pub dividend_per_share_TTM: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyMetrics {
    pub date: String,
    pub revenue_per_share: Option<f64>,
    pub net_income_per_share: Option<f64>,
    pub operating_cash_flow_per_share: Option<f64>,
    pub free_cash_flow_per_share: Option<f64>,
    pub cash_per_share: Option<f64>,
    pub book_value_per_share: Option<f64>,
    pub tangible_book_value_per_share: Option<f64>,
    pub shareholders_equity_per_share: Option<f64>,
    pub interest_debt_per_share: Option<f64>,
    pub market_cap: Option<f64>,
    pub enterprise_value: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub price_to_sales_ratio: Option<f64>,
    pub pocfratio: Option<f64>,
    pub pfcf_ratio: Option<f64>,
    pub pb_ratio: Option<f64>,
    pub ptb_ratio: Option<f64>,
    pub ev_to_sales: Option<f64>,
    pub enterprise_value_over_EBITDA: Option<f64>,
    pub ev_to_operating_cash_flow: Option<f64>,
    pub ev_to_free_cash_flow: Option<f64>,
    pub earnings_yield: Option<f64>,
    pub free_cash_flow_yield: Option<f64>,
    pub debt_to_equity: Option<f64>,
    pub debt_to_assets: Option<f64>,
    pub net_debt_to_EBITDA: Option<f64>,
    pub current_ratio: Option<f64>,
    pub interest_coverage: Option<f64>,
    pub income_quality: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub payout_ratio: Option<f64>,
    pub sales_general_and_administrative_to_revenue: Option<f64>,
    #[serde(rename(serialize = "researchAndDevelopementToRevenue"))]
    pub research_and_ddevelopement_to_revenue: Option<f64>,
    pub intangibles_to_total_assets: Option<f64>,
    pub capex_to_operating_cash_flow: Option<f64>,
    pub capex_to_revenue: Option<f64>,
    pub capex_to_depreciation: Option<f64>,
    pub stock_based_compensation_to_revenue: Option<f64>,
    pub graham_number: Option<f64>,
    pub roic: Option<f64>,
    pub return_on_tangible_assets: Option<f64>,
    pub graham_net_net: Option<f64>,
    pub working_capital: Option<f64>,
    pub tangible_asset_value: Option<f64>,
    pub net_current_asset_value: Option<f64>,
    pub invested_capital: Option<f64>,
    pub average_receivables: Option<f64>,
    pub average_payables: Option<f64>,
    pub average_inventory: Option<f64>,
    pub days_sales_outstanding: Option<f64>,
    pub days_payables_outstanding: Option<f64>,
    pub days_of_inventory_on_hand: Option<f64>,
    pub receivables_turnover: Option<f64>,
    pub payables_turnover: Option<f64>,
    pub inventory_turnover: Option<f64>,
    pub roe: Option<f64>,
    pub capex_per_share: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyMetricsTTM {
    pub revenue_per_share_TTM: Option<f64>,
    pub net_income_per_share_TTM: Option<f64>,
    pub operating_cash_flow_per_share_TTM: Option<f64>,
    pub free_cash_flow_per_share_TTM: Option<f64>,
    pub cash_per_share_TTM: Option<f64>,
    pub book_value_per_share_TTM: Option<f64>,
    pub tangible_book_value_per_share_TTM: Option<f64>,
    pub shareholders_equity_per_share_TTM: Option<f64>,
    pub interest_debt_per_share_TTM: Option<f64>,
    pub market_cap_TTM: Option<f64>,
    pub enterprise_value_TTM: Option<f64>,
    pub pe_ratio_TTM: Option<f64>,
    pub price_to_sales_ratio_TTM: Option<f64>,
    pub pocfratio_TTM: Option<f64>,
    pub pfcf_ratio_TTM: Option<f64>,
    pub pb_ratio_TTM: Option<f64>,
    pub ptb_ratio_TTM: Option<f64>,
    pub ev_to_sales_TTM: Option<f64>,
    pub enterprise_value_over_EBITDATTM: Option<f64>,
    pub ev_to_operating_cash_flow_TTM: Option<f64>,
    pub ev_to_free_cash_flow_TTM: Option<f64>,
    pub earnings_yield_TTM: Option<f64>,
    pub free_cash_flow_yield_TTM: Option<f64>,
    pub debt_to_equity_TTM: Option<f64>,
    pub debt_to_assets_TTM: Option<f64>,
    pub net_debt_to_EBITDATTM: Option<f64>,
    pub current_ratio_TTM: Option<f64>,
    pub interest_coverage_TTM: Option<f64>,
    pub income_quality_TTM: Option<f64>,
    pub dividend_yield_TTM: Option<f64>,
    pub payout_ratio_TTM: Option<f64>,
    pub sales_general_and_administrative_to_revenue_TTM: Option<f64>,
    pub research_and_developement_to_revenue_TTM: Option<f64>,
    pub intangibles_to_total_assets_TTM: Option<f64>,
    pub capex_to_operating_cash_flow_TTM: Option<f64>,
    pub capex_to_revenue_TTM: Option<f64>,
    pub capex_to_depreciation_TTM: Option<f64>,
    pub stock_based_compensation_to_revenue_TTM: Option<f64>,
    pub graham_number_TTM: Option<f64>,
    pub roic_TTM: Option<f64>,
    pub return_on_tangible_assets_TTM: Option<f64>,
    pub graham_net_net_TTM: Option<f64>,
    pub working_capital_TTM: Option<f64>,
    pub tangible_asset_value_TTM: Option<f64>,
    pub net_current_asset_value_TTM: Option<f64>,
    pub invested_capital_TTM: Option<f64>,
    pub average_receivables_TTM: Option<f64>,
    pub average_payables_TTM: Option<f64>,
    pub average_inventory_TTM: Option<f64>,
    pub days_sales_outstanding_TTM: Option<f64>,
    pub days_payables_outstanding_TTM: Option<f64>,
    pub days_of_inventory_on_hand_TTM: Option<f64>,
    pub receivables_turnover_TTM: Option<f64>,
    pub payables_turnover_TTM: Option<f64>,
    pub inventory_turnover_TTM: Option<f64>,
    pub roe_TTM: Option<f64>,
    pub capex_per_share_TTM: Option<f64>,
    pub dividend_per_share_TTM: Option<f64>,
    pub debt_to_market_cap_TTM: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub symbol: Option<String>,
    pub price: Option<f64>,
    pub beta: Option<f64>,
    pub vol_avg: Option<f64>,
    pub mkt_cap: Option<f64>,
    pub last_div: Option<f64>,
    pub range: Option<String>,
    pub changes: Option<f64>,
    pub company_name: Option<String>,
    pub currency: Option<String>,
    pub cik: Option<String>,
    pub isin: Option<String>,
    pub cusip: Option<String>,
    pub exchange: Option<String>,
    pub exchange_short_name: Option<String>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub ceo: Option<String>,
    pub sector: Option<String>,
    pub country: Option<String>,
    pub full_time_employees: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub dcf_diff: Option<f64>,
    pub dcf: Option<f64>,
    pub image: Option<String>,
    pub ipo_date: Option<String>,
    pub default_image: Option<bool>,
    pub is_etf: Option<bool>,
    pub is_actively_trading: Option<bool>,
    pub is_adr: Option<bool>,
    pub is_fund: Option<bool>,
}
pub struct NeededData {
    pub income: (bool, TimePeriod),
    pub income_qtr: (bool, TimePeriod),
    pub balance: (bool, TimePeriod),
    pub ratios: (bool, TimePeriod),
    pub key_metrics: (bool, TimePeriod),
    pub key_metrics_ttm: (bool, TimePeriod),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FetchStats {
    pub last_pull_length: usize,
    pub last_pull_time: Option<NaiveDate>,
}

impl FetchStats {
    pub fn new(last_pull_length: usize) -> Self {
        Self {
            last_pull_length,
            last_pull_time: None,
        }
    }
}

pub trait StockInfo {
    fn length_of_annual_statement(&self) -> usize;
    fn length_of_quarter_statement(&self) -> usize;
    fn recent_annual_date(&self) -> &str;
    fn recent_quarter_date(&self) -> &str;
    fn last_pull_time_annual(&self) -> NaiveDate;
    fn last_pull_time_quarter(&self) -> NaiveDate;
}

impl StockInfo for Statements {
    fn length_of_annual_statement(&self) -> usize {
        self.annual_income.0.len()
    }

    fn length_of_quarter_statement(&self) -> usize {
        self.quarter_income.0.len()
    }

    fn recent_annual_date(&self) -> &str {
        &self.annual_income.0[0].date
    }

    fn recent_quarter_date(&self) -> &str {
        &self.quarter_income.0[0].date
    }

    fn last_pull_time_annual(&self) -> NaiveDate {
        self.annual_income.1.last_pull_time.unwrap()
    }

    fn last_pull_time_quarter(&self) -> NaiveDate {
        self.quarter_income.1.last_pull_time.unwrap()
    }
}

impl StockInfo for Metrics {
    fn length_of_annual_statement(&self) -> usize {
        self.annual_ratios.0.len()
    }

    fn length_of_quarter_statement(&self) -> usize {
        self.quarter_ratios.0.len()
    }

    fn recent_annual_date(&self) -> &str {
        &self.annual_ratios.0[0].date
    }

    fn recent_quarter_date(&self) -> &str {
        &self.quarter_ratios.0[0].date
    }

    fn last_pull_time_annual(&self) -> NaiveDate {
        self.annual_ratios.1.last_pull_time.unwrap()
    }

    fn last_pull_time_quarter(&self) -> NaiveDate {
        self.quarter_ratios.1.last_pull_time.unwrap()
    }
}
