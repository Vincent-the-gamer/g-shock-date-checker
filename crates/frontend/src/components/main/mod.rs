use stylist::{yew::styled_component, Style};
use yew::{Html, html, use_state, Callback, Event};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::components::poster::Poster;

const CSS: &str = grass::include!("crates/frontend/src/components/main/Main.scss");

#[styled_component(Main)]
pub fn main() -> Html{
    let stylesheet = Style::new(CSS).unwrap();
    let code_handle = use_state(String::default);
    let code = (*code_handle).clone();
   

    let code_change = Callback::from(move |e: Event| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            code_handle.set(input.value())
        }
    });

    
    html!{
        <div class={ stylesheet }>
            <main>
              <Poster/>
              <h3>{"Input your 8-digit code on your G-SHOCK watch back: "}</h3>
              <p class="input-holder">
                <input type="text" value={code}
                                   onchange={code_change}
                                   placeholder="input code..."
                                   />
              </p>
              <button>{"Confirm"}</button>
            </main>
        </div>
    }
}