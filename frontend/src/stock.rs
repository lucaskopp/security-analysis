use accounting::Accounting;
use convert_case::{Case, Casing};
use std::vec;

use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
            let mut income = stock[0]["statements"]["annual_income"][0]
                .as_array()
                .unwrap()
                .to_owned();

            let ttm_income = stock[0]["statements"]["ttm_income"][0][0].clone();

            if ttm_income["revenue"].as_f64() != income[0]["revenue"].as_f64() {
                income.insert(0, stock[0]["statements"]["ttm_income"][0][0].clone());
            }

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

            let mut ac = Accounting::new_from("$", 2);
            ac.set_format_positive("{v}");
            ac.set_format_negative("({v})");
            ac.set_format_zero("--");

            html! {
                <>
                    <section class={classes!("container")}>
                        <section>
                            <h2>{ format!("{} ({})", other["profile"][0]["companyName"].as_str().unwrap(), symbol)} <span></span></h2>
                            <p><u>{"Industry: "}</u>{other["profile"][0]["industry"].as_str()}</p>
                        </section>
                        <section>
                            <p>{other["profile"][0]["description"].as_str()}</p>
                            <a href={String::from(other["profile"][0]["website"].as_str().unwrap())}>
                                <img src={String::from(other["profile"][0]["image"].as_str().unwrap())} alt="company logo" />
                            </a>
                        </section>
                        <section>
                            <p><small>{" * Financials in "}<strong>{"Millions"}</strong>{" of "}<strong>{"US Dollar"}</strong></small></p>
                        </section>
                    </section>
                    <section>
                            <table role="grid">
                                <thead>
                                    <tr>
                                        <th scope="col"></th>
                                        {
                                            income.iter().map(|income_statement| html! {
                                                <th scope="col"> <nobr> {income_statement["date"].as_str()}</nobr></th>
                                            }).collect::<Html>()
                                        }
                                    </tr>
                                </thead>
                                <tbody>
                                    {

                                       fields.iter().map(|field| html! {
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
                                                income.iter().map(|income_statement| html! {
                                                    <td>{
                                                            if field.millions == false {
                                                                match income_statement[&field.name_in_api].as_f64() {
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

                                                                match income_statement[&field.name_in_api].as_f64() {

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
