use rocket_dyn_templates::{context, Template};
use rocket::serde::Serialize;
use rocket::serde::json::Json;


#[macro_use] extern crate rocket;
#[get("/")]
fn index() -> Template {
    Template::render("index",context! {camp: "valore"})
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MyReturnStruct {
    key:String
}

#[get("/json-res")]            
fn json_res() -> Json<MyReturnStruct> {  
    Json(MyReturnStruct{
        key:"value".to_string()
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index,json_res])
    .attach(Template::fairing())
}