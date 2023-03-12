use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Screener {
    pub id: usize,
    pub name: AttrValue,
    pub decription: AttrValue,
}

#[derive(Properties, PartialEq)]
pub struct ScreenCardProps {
    pub screen: Screener,
}

#[function_component(ScreenCard)]
pub fn screen_card(ScreenCardProps { screen }: &ScreenCardProps) -> Html {
    html! {
        <article id={screen.id.to_string()}>
            <h5>
                <a href={format!("/screeners/{}", screen.name.clone())}>
                    {screen.name.clone()}
                </a>
            </h5>
            <p>{screen.decription.clone()}</p>
        </article>
    }
}
