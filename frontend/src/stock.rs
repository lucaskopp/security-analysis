use accounting::Accounting;
use std::vec;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlSelectElement};

use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

struct StatementData {
    pub name: String,
    pub field_data: Vec<TableField>,
}

struct TableField {
    pub name_in_api: String,
    pub preffered_name: String,
    pub important: bool,
    pub millions: bool,
    pub negative: bool,
}

#[derive(Properties, PartialEq)]
pub struct StockProps {
    pub symbol: AttrValue,
}

#[function_component(Stock)]
pub fn stock(StockProps { symbol }: &StockProps) -> Html {
    let url = format!("/api/stock/{}", symbol);

    let data = use_state(|| None);
    let statement_data = use_state(|| get_income_statement_meta());

    let on_change = {
        let statement_data = statement_data.clone();

        Callback::from(move |e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

            if let Some(input) = input {
                if input.value() == "income".to_string() {
                    statement_data.set(get_income_statement_meta());
                } else if input.value() == "balance".to_string() {
                    statement_data.set(get_balance_statement_meta());
                } else {
                    statement_data.set(get_cash_statement_meta());
                }
            }
        })
    };

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get(&url).send().await.unwrap();
                    let result: Vec<Value> = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                            .unwrap()
                        } else {
                            resp.json().await.map_err(|err| err.to_string()).unwrap()
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <section class={classes!("container")}>
                    <h1>{symbol}</h1>
                    <progress></progress>
                </section>
            }
        }
        Some(stock) => {
            let other = &stock[0]["other"];
            let stock = stock.clone();

            let statement_data = statement_data.clone();

            let mut statement = stock[0]["statements"][format!("annual_{}", statement_data.name)]
                [0]
            .as_array()
            .unwrap()
            .to_owned();

            let last_income_quarter = &stock[0]["statements"]["quarter_income"][0][0];

            if statement_data.name == "income" {
                let ttm = stock[0]["statements"]["ttm_income"][0][0].clone();

                let last_quarter_period = last_income_quarter["period"].as_str().unwrap();

                if last_quarter_period != "Q4" {
                    statement.insert(0, ttm);
                }
            }

            let mut ac = Accounting::new_from("$", 2);
            ac.set_format_positive("{v}");
            ac.set_format_negative("({v})");
            ac.set_format_zero("--");

            html! {
                <>
                    <section class={classes!("container")}>
                        <section id="top-stock-details">
                            <h2>{ format!("{} ({})", other["profile"][0]["companyName"].as_str().unwrap(), symbol)} <span></span></h2>
                            <p>
                                <u>{"Current Market Valuation:"}</u>{" "}
                                <mark>{market_cap_string(other["profile"][0]["mktCap"].as_f64().unwrap(), &ac)}</mark>
                                {format!(" - ${} per share", ac.format_money(other["profile"][0]["price"].as_f64().unwrap()))}
                            </p>
                            if (other["profile"][0]["price"].as_f64().unwrap() - other["dcf"][0]["equityValuePerShare"].as_f64().unwrap()).abs() / ((other["profile"][0]["price"].as_f64().unwrap() + other["dcf"][0]["equityValuePerShare"].as_f64().unwrap()) / 2.0) < 1.0 {

                            <p>
                                <u>{"Intrinsic Valuation:"}</u>{" "}
                                {market_cap_string(other["dcf"][0]["equityValue"].as_f64().unwrap(), &ac)}
                                {format!(" - ${} per share", ac.format_money(other["dcf"][0]["equityValuePerShare"].as_f64().unwrap()))}
                            </p>
                        }
                            <p><u>{"Exchange:"}</u>{" "}{other["profile"][0]["exchangeShortName"].as_str()}</p>
                            <p><u>{"Sector:"}</u>{" "}{other["profile"][0]["sector"].as_str()}</p>
                            <p><u>{"Industry:"}</u>{" "}{other["profile"][0]["industry"].as_str()}</p>
                            <p><u>{"Country:"}</u>{" "}{other["profile"][0]["country"].as_str()}</p>
                        </section>
                        <section>
                            <p>{other["profile"][0]["description"].as_str()}</p>
                            <a href={String::from(other["profile"][0]["website"].as_str().unwrap())}>
                                <img src={String::from(other["profile"][0]["image"].as_str().unwrap())} alt="company logo" />
                            </a>
                            <small>{" * site"}</small>
                        </section>
                        <section>
                            <select id="statement" onchange={on_change}>
                                <option value="income" selected=true>{"Income Statement"}</option>
                                <option value="balance">{"Balance Sheet Statement"}</option>
                                <option value="cashflow">{"Cash Flow Statement"}</option>
                            </select>
                            <p><small>{" * Financials in "}<strong>{"Millions"}</strong>{" of "}<strong>{last_income_quarter["reportedCurrency"].as_str()}</strong></small></p>
                        </section>
                    </section>
                    <section>
                            <table role="grid">
                                <thead>
                                    <tr>
                                        <th scope="col"></th>
                                        {
                                            statement.iter().map(|s| html! {
                                                <th scope="col"> <nobr> {s["date"].as_str()}</nobr></th>
                                            }).collect::<Html>()
                                        }
                                    </tr>
                                </thead>
                                <tbody>
                                    {

                                       statement_data.field_data.iter().map(|field| html! {
                                        <tr>

                                            if field.important {
                                                <th scope="row">
                                                    <nobr>
                                                        <u><strong>{&field.preffered_name}</strong></u>
                                                    </nobr>
                                                </th>
                                            } else {
                                                <th scope="row"><nobr>{&field.preffered_name}</nobr></th>
                                            }

                                            {
                                                statement.iter().map(|s| html! {
                                                    <td>{
                                                            if field.millions == false {
                                                                match s[&field.name_in_api].as_f64() {
                                                                    Some(v) => {

                                                                        if field.negative == true && v > 0.0 {
                                                                            ac.format_money(v * -1.0)
                                                                        } else {
                                                                            ac.format_money(v)
                                                                        }
                                                                    },
                                                                    None => String::from("N/A"),
                                                                }

                                                            } else {

                                                                match s[&field.name_in_api].as_f64() {

                                                                    Some(v) => {
                                                                        if field.negative == true && v > 0.0 {
                                                                            ac.format_money((v / 1_000_000.0) * -1.0)
                                                                        } else {
                                                                            ac.format_money(v / 1_000_000.0)
                                                                        }

                                                                    },
                                                                    None => String::from("N/A"),
                                                                }
                                                            }

                                                    }</td>
                                                }).collect::<Html>()
                                            }
                                        </tr>
                                       }).collect::<Html>()
                                    }
                                </tbody>
                            </table>
                    </section>
                </>
            }
        }
    }
}

fn market_cap_string(cap: f64, ac: &Accounting) -> String {
    if cap >= 1_000_000_000_000.0 {
        return ac.format_money(cap / 1_000_000_000_000.0) + " Trillion";
    } else if cap >= 1_000_000_000.0 {
        return ac.format_money(cap / 1_000_000_000.0) + " Billion";
    } else {
        return ac.format_money(cap / 1_000_000.0) + " Million";
    }
}

fn get_income_statement_meta() -> StatementData {
    let fields = vec![
        TableField {
            name_in_api: "revenue".to_string(),
            preffered_name: "Total Revenue".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "costOfRevenue".to_string(),
            preffered_name: "Cost of Revenue".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "grossProfit".to_string(),
            preffered_name: "Total Gross Profit".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "researchAndDevelopmentExpenses".to_string(),
            preffered_name: "Research and Developement Expenses".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "generalAndAdministrativeExpenses".to_string(),
            preffered_name: "General and Administrative Expenses".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "sellingAndMarketingExpenses".to_string(),
            preffered_name: "Selling and Marketing Expenses".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "sellingGeneralAndAdministrativeExpenses".to_string(),
            preffered_name: "Selling, General and Administrative Expenses".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "otherExpenses".to_string(),
            preffered_name: "Other Expenses".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "operatingExpenses".to_string(),
            preffered_name: "Total Operating Income/Expenses".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "operatingIncome".to_string(),
            preffered_name: "Total Operating Profit/Loss".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "interestIncome".to_string(),
            preffered_name: "Interest Income".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "interestExpense".to_string(),
            preffered_name: "Interest Expense".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "totalOtherIncomeExpensesNet".to_string(),
            preffered_name: "Total Other Income Expenses Net".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "incomeBeforeTax".to_string(),
            preffered_name: "Income Before Tax".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "incomeTaxExpense".to_string(),
            preffered_name: "Income Tax Expense".to_string(),
            important: false,
            millions: true,
            negative: true,
        },
        TableField {
            name_in_api: "netIncome".to_string(),
            preffered_name: "Net Income".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "eps".to_string(),
            preffered_name: "Basic eps".to_string(),
            important: false,
            millions: false,
            negative: false,
        },
        TableField {
            name_in_api: "epsdiluted".to_string(),
            preffered_name: "Diluted eps".to_string(),
            important: true,
            millions: false,
            negative: false,
        },
        TableField {
            name_in_api: "weightedAverageShsOut".to_string(),
            preffered_name: "Weighted Average Shares Outstanding".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "weightedAverageShsOutDil".to_string(),
            preffered_name: "Weighted Average Shares Outstanding Diluted".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
    ];

    StatementData {
        name: String::from("income"),
        field_data: fields,
    }
}

fn get_balance_statement_meta() -> StatementData {
    let fields = vec![
        TableField {
            name_in_api: "cashAndCashEquivalents".to_string(),
            preffered_name: "Cash and Cash Equivalents".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "shortTermInvestments".to_string(),
            preffered_name: "Short Term Investments".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "cashAndShortTermInvestments".to_string(),
            preffered_name: "Cash Equivalents and Short Term Investments".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "inventory".to_string(),
            preffered_name: "Inventories".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "netReceivables".to_string(),
            preffered_name: "Trade and Other Recievables".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "otherCurrentAssets".to_string(),
            preffered_name: "Other Current Assets".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "totalCurrentAssets".to_string(),
            preffered_name: "Total Current Assets".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "propertyPlantEquipmentNet".to_string(),
            preffered_name: "Net Property, Plant and Equipment".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "longTermInvestments".to_string(),
            preffered_name: "Total Long Term Investments".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "otherNonCurrentAssets".to_string(),
            preffered_name: "Other Non-Current Assets".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "intangibleAssets".to_string(),
            preffered_name: "Net Intangible Assets".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "totalNonCurrentAssets".to_string(),
            preffered_name: "Total Non-Current Assets".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "totalAssets".to_string(),
            preffered_name: "Total Assets".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "accountPayables".to_string(),
            preffered_name: "Payables and Accured Expenses".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "deferredRevenue".to_string(),
            preffered_name: "Deffered Liabilities".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "otherCurrentLiabilities".to_string(),
            preffered_name: "Other Current Liabilities".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "shortTermDebt".to_string(),
            preffered_name: "Financial Liabilities".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "totalCurrentLiabilities".to_string(),
            preffered_name: "Total Current Liabilities".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "longTermDebt".to_string(),
            preffered_name: "Financial Liabilities, Non-Current".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "otherNonCurrentLiabilities".to_string(),
            preffered_name: "Other Non-Current Liabilities".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "deferredRevenueNonCurrent".to_string(),
            preffered_name: "Deffered Liabilities, Non-Current".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "totalNonCurrentLiabilities".to_string(),
            preffered_name: "Total Non-Current Liabilities".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "totalLiabilities".to_string(),
            preffered_name: "Total Liabilities".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "preferredStock".to_string(),
            preffered_name: "Preferred Stock".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "commonStock".to_string(),
            preffered_name: "Common Stock".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "othertotalStockholdersEquity".to_string(),
            preffered_name: "Additional Paid in Capital".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "retainedEarnings".to_string(),
            preffered_name: "Retained Earnings".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "accumulatedOtherComprehensiveIncomeLoss".to_string(),
            preffered_name: "Accumulated Other Comprehensive Income".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "totalEquity".to_string(),
            preffered_name: "Total Equity".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
    ];

    StatementData {
        name: String::from("balance"),
        field_data: fields,
    }
}

fn get_cash_statement_meta() -> StatementData {
    let fields = vec![
        TableField {
            name_in_api: "netIncome".to_string(),
            preffered_name: "Net Income".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "depreciationAndAmortization".to_string(),
            preffered_name: "Depreciation, Amortization and Depletion".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "stockBasedCompensation".to_string(),
            preffered_name: "Stock-Based Compensation".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "deferredIncomeTax".to_string(),
            preffered_name: "Deffered Taxes".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "otherNonCashItems".to_string(),
            preffered_name: "Other Non-Cash Items".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "inventory".to_string(),
            preffered_name: "Change in Inventories".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "otherWorkingCapital".to_string(),
            preffered_name: "Change in Other Current Assets".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "accountsPayables".to_string(),
            preffered_name: "Change in Payables".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "changeInWorkingCapital".to_string(),
            preffered_name: "Total Changes in Operating Capital".to_string(),
            important: false,
            millions: true,
            negative: false,
        },
        TableField {
            name_in_api: "netCashProvidedByOperatingActivities".to_string(),
            preffered_name: "Total Operating Cash Flow".to_string(),
            important: true,
            millions: true,
            negative: false,
        },
    ];

    StatementData {
        name: String::from("cash"),
        field_data: fields,
    }
}
