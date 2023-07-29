mod components;

use yew::{ Html, html, function_component };

use crate::components::{
    header::Header, 
    main::Main
};

#[function_component(App)]
pub fn app() -> Html {
    html!{
        <div>
           <Header/>
           <Main/>
        </div>
    }
}