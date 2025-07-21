use actix_web::{Error, HttpResponse, Responder, web, http::header};
use tera::{Tera, Context};
use validator::{Validate, ValidationErrors};
use serde_json;

use crate::domain::todo::todo;

pub async fn new(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "New To Do - Rust");
    ctx.insert("active_tab", "add_task");
    ctx.insert("errors", &ValidationErrors::new());

    let rendered = tmpl.render("todo/new.html", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn store(tmpl: web::Data<Tera>, form: web::Form<todo::NewToDoRequest>) -> Result<HttpResponse, Error> {
    match form.validate() {
        Ok(_) => {
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, "/"))
                .finish())
        }
        Err(errors) => {
            let errors_value = serde_json::to_value(&errors).unwrap();
            let errors_json = serde_json::to_string(&errors).unwrap();

            let mut ctx = Context::new();
            ctx.insert("errors", &errors_value);
            ctx.insert("errors_json", &errors_json);
            ctx.insert("form_values", &form);
            ctx.insert("title", "New To Do - Rust");
            ctx.insert("active_tab", "add_task");
            
            let rendered = tmpl.render("todo/new.html", &ctx).unwrap();

            Ok(
                HttpResponse::BadRequest()
                .content_type("text/html")
                .body(rendered)
            )
        }
    }
}