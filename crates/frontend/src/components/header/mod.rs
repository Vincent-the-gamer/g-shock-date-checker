use stylist::Style;
use yew::{ Html, html, function_component };

const CSS: &str = grass::include!("crates/frontend/src/components/header/Header.scss");

#[function_component(Header)]
pub fn header() -> Html {
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
          <header>
            <img src="assets/img/logo.jpg" alt="logo"/>
            <h3>{"G-Shock Date Checker"}</h3>
            <div class="right">
              <div class="github">
                <a href="https://github.com/Vincent-the-gamer/g-shock-date-checker" target="_blank">
                  <img src="assets/img/github.png" alt="logo"/>
                  <h2>{"GitHub"}</h2>
                </a>
              </div>
            </div>
          </header>
        </div>
    }
}