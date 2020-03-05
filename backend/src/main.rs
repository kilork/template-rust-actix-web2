use actix_web::{App, HttpServer};
#[cfg(feature = "ui")]
use actix_web_static_files;

use std::collections::HashMap;

#[cfg(feature = "ui")]
use {{crate_name}}_frontend::generate;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let mut app = App::new();
        #[cfg(feature = "ui")]
        {
            let generated = generate();
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
