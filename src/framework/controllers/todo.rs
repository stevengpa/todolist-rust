use actix_web::{HttpResponse, Responder, web};
use tera::Tera;

pub async fn new(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "New To Do - Rust");
    ctx.insert("active_tab", "add_task");

    let rendered = tmpl.render("todo/new.html", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}