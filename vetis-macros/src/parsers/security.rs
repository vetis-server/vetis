use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Result, Token,
};

pub(crate) struct SecurityArgs {
    pub(crate) cert: Option<Expr>,
    pub(crate) key: Option<Expr>,
    pub(crate) ca_cert: Option<Expr>,
    pub(crate) client_auth: Option<Expr>,
}

impl Parse for SecurityArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut cert_file = None;
        let mut key_file = None;
        let mut ca_cert = None;
        let mut client_auth = None;

        // Loop through the comma-separated key:value pairs
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;

            // Check which key we found and look for duplicates
            match key
                .to_string()
                .as_str()
            {
                "cert" => {
                    if cert_file.is_some() {
                        return Err(input.error("Duplicate 'cert' key"));
                    }
                    let val: Expr = input.parse()?;
                    cert_file = Some(val);
                }
                "key" => {
                    if key_file.is_some() {
                        return Err(input.error("Duplicate 'key' key"));
                    }
                    let val: Expr = input.parse()?;
                    key_file = Some(val);
                }
                "ca_cert" => {
                    if ca_cert.is_some() {
                        return Err(input.error("Duplicate 'ca_cert' key"));
                    }
                    let val: Expr = input.parse()?;
                    ca_cert = Some(val);
                }
                "client_auth" => {
                    if client_auth.is_some() {
                        return Err(input.error("Duplicate 'client_auth' key"));
                    }
                    let val: Expr = input.parse()?;
                    client_auth = Some(val);
                }
                _ => return Err(input.error(format!("Unknown key: {}", key))),
            }

            // If there's a comma, consume it and continue; otherwise, we're done
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(SecurityArgs { cert: cert_file, key: key_file, ca_cert, client_auth })
    }
}
