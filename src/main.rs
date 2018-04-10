#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use std::collections::HashMap;


#[derive(FromForm)]
struct UserInput {
    greeting: String
}

#[get("/greet/<target>")]
fn greet(target: String) -> Template {
    let mut context = HashMap::new();
    context.insert("target", target);

    Template::render("greeter", &context)
}

#[post("/greet", data = "<target>")]
fn greet_form(target: Form<UserInput>) -> Redirect {
    let inp = target.into_inner();
    let mut strg = String::from("/greet/");

    strg.push_str(&inp.greeting);
    Redirect::to(&strg)
}

#[get("/")]
fn index() -> Template{
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![greet, greet_form, index])
        .attach(Template::fairing())
        .launch();
}
