mod cli;

pub struct Config {
    local_echo: bool,
}

impl Config {
    pub fn new(_args: &[String]) -> Result<Config, &str> {
        Ok(Config { local_echo: true })
    }
}

pub fn run_cli(config: Config) {
    // Do not take config into accout for the moment
    cli::run(config);
}
