use actix_web::{App, HttpServer};
use actix_web_static_files;

use std::collections::HashMap;

use {{project-name}}_frontend::generate;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        let mut app = App::new();
        {
            app = app.service(actix_web_static_files::ResourceFiles::new(
                "/", generated,
            ));
        }
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
