mod components;

use stylist::{yew::styled_component};
use yew::{ Html, html };

use crate::components::{
    header::Header, 
    main::Main, 
    footer::Footer
};

#[styled_component(App)]
pub fn app() -> Html {
    html!{
        <div>
           <Header/>
           <Main/>
           <Footer/>
        </div>
    }
}