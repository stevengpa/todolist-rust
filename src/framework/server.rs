use crate::framework::controllers::{task, todo, not_found};
use actix_web::{App, HttpServer, web};
use tera::Tera;

pub async fn server(host: &str, port: u16) -> std::io::Result<()> {
    let mut tera = Tera::new("templates/**/**").unwrap();
    tera.full_reload().unwrap();
    tera.autoescape_on(vec![".html", ".sql"]);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(web::resource("/").route(web::get().to(task::list)))
            .service(web::resource("/todo/new").route(web::get().to(todo::new)))
            .default_service(web::route().to(not_found::index))
    })
    .bind((host, port))?
    .run()
    .await
}
