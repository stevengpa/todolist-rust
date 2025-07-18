use actix_web::HttpResponse;
use actix_web::{Responder, web};

use tera::Tera;

pub async fn list(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "To Do List - Rust");
    ctx.insert("active_tab", "tasks");

    let rendered = tmpl.render("task/list.html", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}
