use vetis_macros::security;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = security!{
        cert => "../../../../certs/server.der",
        key => "../../../../certs/server.key.der",
        ca_cert => "../../../../certs/ca.der",
        client_auth => true,
    };

    Ok(())
}
