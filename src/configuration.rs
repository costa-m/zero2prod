#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    println!("{}", base_path.display());
    let configuration_directory = base_path.join("src").join("configuration.yaml");
    // Initialize our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configurations.yaml`
        .add_source(config::File::from(configuration_directory))
        .build()?;

    // Try to convert the configuration files it reads into Settings type
    settings.try_deserialize::<Settings>()
}
