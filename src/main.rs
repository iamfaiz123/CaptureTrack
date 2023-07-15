use ScreenShotTool::{
    errors::core_error::CoreError,
    service::{config_loader::config_loader, server_loader::Application},
};

#[actix_web::main]
async fn main() -> Result<(), CoreError> {
    let config = config_loader()?;
    let app = Application::new(config).await?;
    let _ = app.run_server().await;
    Ok(())
}
