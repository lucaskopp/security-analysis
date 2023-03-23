use std::str;

use gloo_net::http::Request;
use serde_json::Value;
use stock::Stock;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::screener::{ScreenCard, Screener};

mod screener;
mod stock;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/screeners")]
    ScreenersPage,
    #[at("/screeners/*name")]
    ScreenPage { name: String },
    #[at("/stock/*symbol")]
    StockPage { symbol: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <h1>{ "Rust fullstack demo" }</h1> }
        }
        Route::ScreenersPage => html! {
            <ScreenersPage />
        },
        Route::ScreenPage { name } => html! {
            <ScreenPage name={AttrValue::from(name)} />
        },
        Route::StockPage { symbol } => html! {
            <Stock symbol={AttrValue::from(symbol)} />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <header class={classes!("container")}>
                    <hgroup>
                        <nav>
                            <ul>
                                <li>
                                    <h1>
                                        <a href="/"><kbd>{"sc_an"}</kbd> </a>
                                    </h1>
                                 </li>
                            </ul>
                            <ul>
                                <li>
                                    <details role="list" dir="rtl">
                                        <summary aria-haspopup="listbox" role="link"></summary>
                                        <ul role="listbox">
                                            <li><a href="/">{"Home"}</a></li>
                                            <li><a href="/screeners">{"Screeners"}</a></li>
                                        </ul>
                                    </details>
                                </li>
                                <li>
                                    <input type="search" id="search" name="search" placeholder="Search Symbols" />
                                </li>
                            </ul>
                        </nav>
                    </hgroup>
            </header>
            <main>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </main>
        </>
    }
}

// #[function_component(HelloServer)]
// fn hello_server() -> Html {
//     let data = use_state(|| None);

//     // Request `/api/hello` once
//     {
//        let data = data.clone();
//         use_effect(move || {
//             if data.is_none() {
//                 spawn_local(async move {
//                     let resp = Request::get("/api/hello").send().await.unwrap();
//                     let result = {
//                         if !resp.ok() {
//                             Err(format!(
//                                 "Error fetching data {} ({})",
//                                 resp.status(),
//                                 resp.status_text()
//                             ))
//                         } else {
//                             resp.text().await.map_err(|err| err.to_string())
//                         }
//                     };
//                     data.set(Some(result));
//                 });
//             }

//             || {}
//         });
//     }

//     match data.as_ref() {
//         None => {
//             html! {
//                 <div>{"No server response"}</div>
//             }
//         }
//         Some(Ok(data)) => {
//             html! {
//                 <div>{"Got server response: "}{data}</div>
//             }
//         }
//         Some(Err(err)) => {
//             html! {
//                 <div>{"Error requesting data from server: "}{err}</div>
//             }
//         }
//     }
// }

#[function_component(ScreenersPage)]
fn screeners_page() -> Html {
    let screen = Screener {
        id: 0,
        name: AttrValue::from("Buffetology"),
        decription: AttrValue::from("The Buffett strategy looks for stocks for an extremely 
        long term horizon and combines both value and quality factors to identify stocks of companies with solid businesses 
        and profitability and sound financials that trade at an attractive prices. Only stocks with consistent long term track records can 
        pass this methodology."),
    };

    html! {
        <section class={classes!("container")}>
            <h1>{"Screeners"}</h1>
            <ScreenCard screen={screen}></ScreenCard>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct ScreenPageProps {
    pub name: AttrValue,
}

#[function_component(ScreenPage)]
fn screen_page(ScreenPageProps { name }: &ScreenPageProps) -> Html {
    let url = format!("/api/screeners/{}", name);

    let data = use_state(|| None);

    // Request `/api/hello` once
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
                    <h1>{name}</h1>
                    <progress></progress>
                </section>
            }
        }
        Some(v) => {
            html! {
                <section class={classes!("container")}>
                    <h1>{name}</h1>
                    <table role="grid">
                        <thead>
                            <tr>
                                <th scope="col"><strong>{"Symbol"}</strong></th>
                            </tr>
                        </thead>
                        <tbody>
                            {v.iter().map(|s| html! {
                                <tr>
                                    <td><a href={format!("/stock/{}", s.as_str().unwrap())}>{s.as_str()}</a></td>
                                </tr>
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                </section>
            }
        }
    }
}

// fn on_run(name: String) -> Vec<Value> {
//     let res : Vec<Value> = Request::get("/api/screeners/")
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
