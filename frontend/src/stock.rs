use accounting::Accounting;
use convert_case::{Case, Casing};
use std::vec;

use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
            let income = &stock[0]["statements"]["annual_income"][0]
                .as_array()
                .unwrap();

            let keys = vec![
                "revenues",
                "costOfRevenue",
                "grossProfit",
                "grossProfitRatio",
                "researchAndDevelopmentExpenses",
                "generalAndAdministrativeExpenses",
                "sellingAndMarketingExpenses",
                "sellingGeneralAndAdministrativeExpenses",
                "otherExpenses",
                "operatingExpenses",
                "costAndExpenses",
                "interestIncome",
                "interestExpense",
                "depreciationAndAmortization",
                "ebitda",
                "ebitdaratio",
                "operatingIncome",
                "operatingIncomeRatio",
                "totalOtherIncomeExpensesNet",
                "incomeBeforeTax",
                "incomeBeforeTaxRatio",
                "incomeTaxExpense",
                "netIncome",
                "netIncomeRatio",
                "eps",
                "dilutedEPS",
                "weightedAverageShsOut",
                "weightedAverageShsOutDil",
            ];

            let important = vec![
                "revenues",
                "grossProfit",
                "operatingExpenses",
                "operatingIncome",
                "netIncome",
                "dilutedEPS",
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

                                       keys.iter().map(|key| html! {
                                        <tr>

                                            if important.contains(key) {
                                                <th scope="row">
                                                    <nobr>
                                                        <u><strong>{key.to_case(Case::Title)}</strong></u>
                                                    </nobr>
                                                </th>
                                            } else {
                                                <th scope="row"><nobr>{key.to_case(Case::Title)}</nobr></th>
                                            }


                                            {
                                                income.iter().map(|income_statement| html! {
                                                    <td>{

                                                            if key.to_case(Case::Title).contains("Ratio") ||  key.to_case(Case::Title).contains("Eps") {

                                                                match income_statement[key].as_f64() {
                                                                    Some(v) => ac.format_money(v),
                                                                    None => String::from("N/A"),
                                                                }

                                                            } else {

                                                                match income_statement[key].as_f64() {
                                                                    Some(v) => ac.format_money(v / 1_000_000.0),
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
