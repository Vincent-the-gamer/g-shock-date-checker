use stylist::{yew::styled_component, Style};
use yew::{Html, html, use_state, Callback, Event};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, MouseEvent};
use crate::components::poster::Poster;
use code_parser::code_parser;

const CSS: &str = grass::include!("crates/frontend/src/components/main/Main.scss");

#[styled_component(Main)]
pub fn main() -> Html{
    let stylesheet = Style::new(CSS).unwrap();
    let code_handle = use_state(String::default);

    let code_handle_clone = code_handle.clone();
    let code = (*code_handle).clone();
 
    let search_result_handle = use_state(String::default);
    let search_result = (*search_result_handle).clone();
   
    let code_change = Callback::from(move |e: Event| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            code_handle_clone.set(input.value())
        }
    });

    let confirm_click: Callback<MouseEvent> = Callback::from(move |_| {
        let result_json = code_parser::parse(code.as_str());
        let location = result_json["location"].as_str();
        
        let result = match location {
            Some(location) => {
                let production_date = result_json["productionDate"].as_str().unwrap();
                format!(
                    "Factory location(所属工厂): {}\nProduction Date(生产日期): {}\n\n如果查询出多个年份，请根据您G-Shock的发布日推断具体年份。\nIf more than 1 production date showed, please infer the specific date from the release date of your G-SHOCK watch.", 
                    location, production_date
                )
            },
            None => {
                let msg = result_json["msg"].as_str();
                if let Some(msg) = msg {
                    String::from(msg)
                }
                else {
                    String::from("Error!")
                }
            }
        };

        search_result_handle.set(result);
    });

    let code_handle_clone = code_handle.clone();
    let clear_code: Callback<MouseEvent> = Callback::from(move |_| {
        code_handle_clone.set(String::default());
    });

    let code = (*code_handle).clone();
    
    html!{
        <div class={ stylesheet }>
            <main>
              <Poster/>
              <h3>{"Please input your 8-digit code on your G-SHOCK watch back: "}</h3>
              <h3>{"请输入表壳后面的8位编码: "}</h3>
              <p class="input-holder">
                <input type="text" value={code}
                                   onchange={code_change}
                                   placeholder="输入编码 input code..."
                                   />
                <button onclick={confirm_click}>{"确认 Confirm"}</button>
                <button class="clear" onclick={clear_code}>{"清空 Clear"}</button>
              </p>
              <div class="result">
                <h3>{"Result(查询结果):"}</h3>
                <h3>{&search_result}</h3>
              </div>
              <div class="content-holder">
                <h4>{"Where is the code? Find it on the watch back!"}</h4>
                <h4>{"8位编码在何处？牧童遥指表壳后~~"}</h4>
                <p class="img-holder"> 
                    <img src="assets/img/g-shock-code.jpg" alt="code"/>
                </p>
                <h4>{"What's the code stands for? See it in GitHub."}</h4>
                <h4>{"想知道这个代码代表啥？看我的GitHub吧。"}</h4>
                <a href="https://github.com/Vincent-the-gamer/g-shock-date-checker" target="_blank">
                    <h2>{"My GitHub"}</h2>
                </a>
              </div>
            </main>
        </div>
    }
}