use stylist::{yew::styled_component, Style};
use yew::{Html, html};

const CSS: &str = grass::include!("crates/frontend/src/components/main/Main.scss");

#[styled_component(Main)]
pub fn main() -> Html{
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={ stylesheet }>
            <main>
              <h2>{"Main"}</h2>
            </main>
        </div>
    }
}