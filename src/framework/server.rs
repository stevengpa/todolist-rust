use crate::framework::controllers::{task, todo};
use actix_web::{App, HttpServer, web};
use tera::Tera;

pub async fn server(host: &str, port: u16) -> std::io::Result<()> {
    let mut tera = Tera::new("templates/**/**").unwrap();
    tera.full_reload().unwrap();
    tera.autoescape_on(vec![".html", ".sql"]);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(task::list))
            .route("/todo/new", web::get().to(todo::new))
    })
    .bind((host, port))?
    .run()
    .await
}
