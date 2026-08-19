use clap::Parser;
use log::error;
#[cfg(target_env = "musl")]
use mimalloc::MiMalloc;
use serde::Deserialize;
#[global_allocator]
#[cfg(target_env = "musl")]
static GLOBAL: MiMalloc = MiMalloc;
use std::{error::Error, fs::read_to_string, path::Path};
use vetis::{host::HostConfig, server::ServerConfig, VetisServer as _};
use vetis_tokio::{host::HostImpl, Vetis};

#[derive(Deserialize)]
pub struct VetisServerConfig {
    log_level: String,
    worker_threads: usize,
    max_blocking_threads: usize,
    server: ServerConfig,
    hosts: Vec<HostConfig>,
}

#[derive(Parser)]
#[command(
    name = "vetis",
    about = "vetis - a very tiny server",
    long_about = r#"
vetis - a very tiny server

Usage:
    vetis [OPTIONS]

Options:
    -h, --help       Print help information
    -V, --version    Print version information
    -c, --config     <CONFIG>
                     Config file to use
"#
)]
struct Args {
    #[arg(short, long, required = false, help = "Config file to use.")]
    config: Option<String>,
}

async fn run(
    server_config: ServerConfig,
    hosts_config: Vec<HostConfig>,
) -> Result<(), Box<dyn Error>> {
    let mut server = Vetis::new(server_config);

    for host in hosts_config {
        let host = HostImpl::new(host);

        server
            .add_host(host)
            .await;
    }

    if let Err(e) = server.run().await {
        error!("Failed to start server: {}", e);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if let Some(config) = args.config {
        if Path::exists(Path::new(&config)) {
            let file = read_to_string(&config);
            if let Ok(file) = file {
                let config = serde_yaml_ng::from_str::<VetisServerConfig>(&file);
                if let Ok(config) = config {
                    env_logger::Builder::from_env(
                        env_logger::Env::default().filter_or("RUST_LOG", config.log_level),
                    )
                    .format_module_path(false)
                    .target(env_logger::Target::Stdout)
                    .init();

                    let rt = tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .worker_threads(config.worker_threads)
                        .max_blocking_threads(config.max_blocking_threads)
                        .build()?;
                    rt.block_on(async { run(config.server, config.hosts).await })?;
                } else {
                    eprintln!(
                        "Failed to start server: {}",
                        config
                            .err()
                            .unwrap()
                    );
                }
            } else {
                eprintln!("Failed to start server: {}", config);
            }
        } else {
            eprintln!("Failed to start server: Config file does not exist: {}", config);
        }
    } else {
        eprintln!("Failed to start server: No config file specified");
    }

    Ok(())
}
