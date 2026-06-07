use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Result, Token,
};

pub(crate) struct HttpArgs {
    pub(crate) protocol: Option<Expr>,
    pub(crate) handler: Option<Expr>,
    pub(crate) from_crate: Option<Ident>,
    pub(crate) hostname: Option<Expr>,
    pub(crate) root_directory: Option<Expr>,
    pub(crate) port: Option<Expr>,
    pub(crate) interface: Option<Expr>,
    pub(crate) security: Option<Expr>,
}

impl Parse for HttpArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut protocol = None;
        let mut handler = None;
        let mut from_crate = None;
        let mut hostname = None;
        let mut root_directory = None;
        let mut port = None;
        let mut interface = None;
        let mut security = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;

            match key
                .to_string()
                .as_str()
            {
                "protocol" => {
                    if protocol.is_some() {
                        return Err(input.error("Duplicate 'protocol' key"));
                    }
                    let expr: Expr = input.parse()?;
                    protocol = Some(expr);
                }
                "handler" => {
                    if handler.is_some() {
                        return Err(input.error("Duplicate 'handler' key"));
                    }
                    let expr: Expr = input.parse()?;
                    handler = Some(expr);
                }
                "from_crate" => {
                    if from_crate.is_some() {
                        return Err(input.error("Duplicate 'from_crate' key"));
                    }
                    let ident: Ident = input.parse()?;
                    from_crate = Some(ident);
                }
                "hostname" => {
                    if hostname.is_some() {
                        return Err(input.error("Duplicate 'hostname' key"));
                    }
                    let expr: Expr = input.parse()?;
                    hostname = Some(expr);
                }
                "root_directory" => {
                    if root_directory.is_some() {
                        return Err(input.error("Duplicate 'root_directory' key"));
                    }
                    let expr: Expr = input.parse()?;
                    root_directory = Some(expr);
                }
                "port" => {
                    if port.is_some() {
                        return Err(input.error("Duplicate 'port' key"));
                    }
                    let expr: Expr = input.parse()?;
                    port = Some(expr);
                }
                "interface" => {
                    if interface.is_some() {
                        return Err(input.error("Duplicate 'interface' key"));
                    }
                    let expr: Expr = input.parse()?;
                    interface = Some(expr);
                }
                "security_config" => {
                    if security.is_some() {
                        return Err(input.error("Duplicate 'security' key"));
                    }
                    let expr: Expr = input.parse()?;
                    security = Some(expr);
                }
                _ => return Err(input.error(format!("Unknown key: {}", key))),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(HttpArgs {
            protocol,
            handler,
            from_crate,
            hostname,
            root_directory,
            port,
            interface,
            security,
        })
    }
}
