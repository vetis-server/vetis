use vetis_macros::security;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = security!{
        cert => concat!(env!("CARGO_WORKSPACE_DIR"), "/certs/server.der"),
        key => concat!(env!("CARGO_WORKSPACE_DIR"), "/certs/server.key.der"),
        ca_cert => concat!(env!("CARGO_WORKSPACE_DIR"), "/certs/ca.der"),
        client_auth => true,
    };

    Ok(())
}
