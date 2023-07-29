mod components;

use yew::{ Html, html, function_component };

use crate::components::Header::Header;

#[function_component(App)]
pub fn app() -> Html {
    html!{
        <div>
           <Header/>
           <h1>{"To be continued..."}</h1>
        </div>
    }
}