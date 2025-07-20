use crate::framework::controllers::{task, todo, not_found};
use actix_web::{App, HttpServer, web, middleware::Logger};
use tera::Tera;

fn init_tera() -> Tera {
    let mut tera = Tera::new("templates/**/**").unwrap();
    tera.full_reload().unwrap();
    tera.autoescape_on(vec![".html", ".sql"]);
    tera
}

pub async fn server(host: &str, port: u16) -> std::io::Result<()> {
    let tera = init_tera();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .service(
                web::resource("/")
                    .route(web::get().to(task::list))
            )
            .service(
                web::scope("/todo")
                    .route("/new", web::get().to(todo::new))
                    .route("/new", web::post().to(todo::store))
            )
            .default_service(web::route().to(not_found::index))
    })
    .bind((host, port))?
    .run()
    .await
}
