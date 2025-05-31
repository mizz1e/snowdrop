use self::module::Module;
use tracing::{error, info};

mod library;
mod module;
mod x86;

fn main() {
    tracing_subscriber::fmt::init();

    if let Err(error) = run() {
        error!("{error}");
    }
}

fn run() -> Result<(), String> {
    let client = Module::open("libclient.so")
        .or_else(|_error| Module::open("client_client.so"))
        .map_err(|_error| String::from("no supported client found"))?;

    info!("loaded client for Source {}", client.version());

    Ok(())
}
