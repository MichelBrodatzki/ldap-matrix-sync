use clap::Parser;
use env_logger;
use log::warn;

mod ldap_matrix_sync;
mod ldap;
mod matrix;

/// This program syncs LDAP group memberships with Matrix chat room memberships
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config: String,
    /// Program won't modify config file, if this flag is set
    #[arg(long, default_value_t = false)]
    read_only: bool,
    /// Enables verbose logging
    #[arg(long, short, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args: Args = Args::parse();

    env_logger::Builder::from_env(
            env_logger::Env::default()
                .default_filter_or(if args.verbose { "trace" } else { "warn" })
        )
        .try_init()
        .unwrap_or_else(|_| {
            warn!("env_logger was already initialized!");
        });

    ldap_matrix_sync::run(args.config, args.read_only);
}
