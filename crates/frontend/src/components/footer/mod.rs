use stylist::{yew::styled_component, Style};
use yew::{Html, html};

const CSS: &str = grass::include!("crates/frontend/src/components/footer/Footer.scss");

#[styled_component(Footer)]
pub fn footer() -> Html{
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
            <footer>
                <h2>{"v1.0.0 | @2023-present Vincent-the-gamer | MIT Licensed"}</h2>           
            </footer>
        </div>
    }
}