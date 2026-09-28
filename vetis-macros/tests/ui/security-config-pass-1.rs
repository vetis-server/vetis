use vetis_macros::tls;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = tls!{
        cert => "../../../../certs/server.der",
        key => "../../../../certs/server.key.der",
        ca_cert => "../../../../certs/ca.der",
        client_auth => true,
    };

    Ok(())
}
