use std::collections::HashMap;
use actix_cors::Cors;
use actix_web::{App, HttpServer, get, web::Query, Responder, HttpResponse};
use code_parser::code_parser;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = 8081;
   
    println!("App running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
        .service(get_g_shock_info)
        // actix-web cors
        .wrap(Cors::default()
             .allow_any_header()
             .allow_any_origin()
             .allow_any_method()
        )  
    })
        .bind((host, port))?
        .run()
        .await
}

#[get("/")]
async fn get_g_shock_info(query: Query<HashMap<String, String>>) -> impl Responder {
     let code = query.get("code");
     let response = match code {
         Some(value) => {
            code_parser::parse(value)
         },
         None => serde_json::from_str(r#"
         {{
            "msg": "妹有获取到任何东西！ You get nothing!"
         }}
         "#).unwrap()
     };

     HttpResponse::Ok().json(response)
}