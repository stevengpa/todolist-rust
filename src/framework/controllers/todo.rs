use actix_web::{Error, HttpResponse, Responder, web, http::header};
use tera::Tera;
use serde::Deserialize;

pub async fn new(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "New To Do - Rust");
    ctx.insert("active_tab", "add_task");

    let rendered = tmpl.render("todo/new.html", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[derive(Deserialize)]
pub struct StoreTaskForm {
    task: String,
    description: String,
}

pub async fn store(form: web::Form<StoreTaskForm>) -> Result<HttpResponse, Error> {
    println!("task: {:?}", form.task);
    println!("description: {:?}", form.description);
    // https://github.com/SeaQL/sea-orm/blob/master/examples/actix_example/api/src/lib.rs
    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish())
}