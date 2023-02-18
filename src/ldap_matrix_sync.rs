use log::{debug};

pub fn run(config_file: String, read_only: bool) {
    debug!("Config file: {config_file}");
    debug!("Read only: {}", if read_only { "yes" } else { "no" });
}