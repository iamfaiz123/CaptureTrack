use crate::errors::core_error::CoreError;
use anyhow::Context;
use config::File;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct DatabaseConfig {
    port: u16,
    host: String,
    password: String,
    name: String,
    connection_url: String,
}

impl DatabaseConfig{
    pub fn get_connection_url(&self)->&str{
        &self.connection_url
    }
}


#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct ApplicationConfig {
    #[serde(rename(deserialize = "application_host"))]
    pub host: String,
    #[serde(rename(deserialize = "application_port"))]
    pub port: u16,
    pub database: DatabaseConfig,
}

pub fn config_loader() -> Result<ApplicationConfig, CoreError> {
    // get current dir
    let current_dir = std::env::current_dir().context("can not get current dir")?;
    let run_mode = std::env::args();
    let config_file_name: String;
    let run_mode: Vec<String> = run_mode.collect();
    unsafe {
        let mode = run_mode.get_unchecked(1);
        match mode.as_str() {
            "dev" | "DEV " => {
                config_file_name = "dev_config.json".into();
            }
            "prod" | "PROD" => {
                config_file_name = "prod_config.json".into();
            }
            // if args is not supplied the run in DEV mode
            _ => {
                config_file_name = "dev_config.json".into();
            }
        }
    }

    let config_file_path = current_dir.join(config_file_name);
    let application_config: ApplicationConfig = config::Config::builder()
        .add_source(File::from(config_file_path))
        .build()?
        .try_deserialize()?;
    Ok(application_config)
}
