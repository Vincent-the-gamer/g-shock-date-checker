use stylist::Style;
use yew::{ Html, html, function_component};

const CSS: &str = grass::include!("crates/frontend/src/components/header/Header.scss");

#[function_component(Header)]
pub fn header() -> Html {
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
          <h1>{"I'm Header"}</h1>
        </div>
    }
}