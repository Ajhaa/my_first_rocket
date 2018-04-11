use rocket::request::Form;
use rocket::response::Redirect;
use rocket::Route;
use rocket_contrib::Template;
use std::collections::HashMap;

#[derive(FromForm)]
struct UserInput {
    target: String,
}

#[get("/<target>")]
fn greet(target: String) -> Template {
    let mut context = HashMap::new();
    context.insert("target", target);

    Template::render("greeter", &context)
}

#[get("/")]
fn greeter_form() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("greeter_form", context)
}

#[post("/", data = "<target>")]
fn handle_greeting(target: Form<UserInput>) -> Redirect {
    let inp = target.into_inner();
    let mut strg = String::from("/greet/");

    strg.push_str(&inp.target);
    Redirect::to(&strg)
}

pub fn get_routes() -> Vec<Route> {
    routes![greet, greeter_form, handle_greeting]
}
