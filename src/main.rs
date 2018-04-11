#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod routers;
use rocket_contrib::Template;
use routers::greeter_router::get_routes;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", context)
}

fn main() {
    let greet_routes = get_routes();
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/greet", greet_routes)
        .attach(Template::fairing())
        .launch();
}
