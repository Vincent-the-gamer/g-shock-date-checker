use stylist::{yew::styled_component, Style};
use yew::{Html, html};

const CSS: &str = grass::include!("crates/frontend/src/components/poster/Poster.scss");

#[styled_component(Poster)]
pub fn poster() -> Html{
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
            <div class="poster">
                <img src="assets/img/poster.png" alt="poster"/>
            </div>
        </div>
    }
}