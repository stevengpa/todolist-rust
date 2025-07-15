use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use tera::Tera;

async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "To Do List");

    let rendered = tmpl.render("index.html", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn server() -> std::io::Result<()> {
    let mut tera = Tera::new("templates/**/**").unwrap();
    tera.autoescape_on(vec![".html", ".sql"]);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
