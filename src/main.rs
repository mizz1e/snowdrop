use self::module::Module;

mod library;
mod module;
mod x86;

fn main() {
    if let Err(error) = run() {
        eprintln!("snowdrop: {error}");
    }
}

fn run() -> Result<(), String> {
    source2()
        .or_else(|_error| source1())
        .map_err(|error| format!("no supported client: {error}"))
}

fn source2() -> Result<(), String> {
    let module = Module::open("libclient.so")?;

    for interface in module.interfaces() {
        println!("{:?}", interface);
    }

    Ok(())
}

fn source1() -> Result<(), String> {
    let module = Module::open("client_client.so")?;

    for interface in module.interfaces() {
        println!("{:?}", interface);
    }

    Ok(())
}
