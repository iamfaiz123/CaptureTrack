use crate::api::v1::api_cfg;
use crate::service::config_loader::ApplicationConfig;
use diesel::connection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
use std::thread;
use actix_web::{web, App, HttpResponse, HttpServer};
pub struct Application {
    server: Option<actix_web::dev::Server>,
    settings: ApplicationConfig,
}

impl Application {
    pub async fn new(settings: ApplicationConfig) -> Result<Self, anyhow::Error> {
        let listener =
            std::net::TcpListener::bind(format!("{}:{}", &settings.host, &settings.port))?;
        //let server = Self::build_server(listener)?;
        let mut application = Self {
            server: None,
            settings: settings,
        };
        // set server in application
        let _ = application.set_server(listener)?;
        Ok(application)
    }

    pub fn get_connection_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        let url = self.get_database_connect_url();
        // make a connection pool to database  
        let manager = ConnectionManager::<PgConnection>::new(url);
        // Refer to the `r2d2` documentation for more methods to use
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            // don't handle error, panic if connection to database can not be made
            .expect("Could not build connection pool")
    }

    fn get_database_connect_url(&self)->&str{
        self.settings.database.get_connection_url()
    }

    fn set_server(
        &mut self,
        listener: std::net::TcpListener,
    ) -> Result<(), std::io::Error> {

        // get pg connection pool
        let db_pool = self.get_connection_pool();
        // wrap connection pool in actix_web::data
        let connection_pool = web::Data::new(db_pool);
        let server = HttpServer::new(move|| {
            App::new()
                .default_service(web::route().to(HttpResponse::MethodNotAllowed))
                .app_data(connection_pool.clone())
                .route(
                    "/",
                    web::get().to(move || async { "hello there i am up and alive" }),
                )
                .service(
                    // load api config from api folder
                    web::scope("/api").configure(api_cfg),
                )
        })
        .listen(listener)?
        .run();
        self.server = Some(server);
        Ok(())
    }

    // call this to run server
    pub async fn run_server(self) -> ::std::io::Result<()> {
        if let None = self.server{
            // panic for now
            panic!(" set server must be called first before running the server");
        }
        self.server.unwrap().await
    }
}
