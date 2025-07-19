use actix_web::{HttpResponse, Responder, web};
use tera::Tera;

pub async fn index(tmpl: web::Data<Tera>) -> impl Responder {
	let mut ctx = tera::Context::new();
	ctx.insert("title", "404 - Rust");
	ctx.insert("active_tab", "");

	let rendered = tmpl.render("404/index.html", &ctx).unwrap();

	HttpResponse::Ok().content_type("text/html").body(rendered)
}