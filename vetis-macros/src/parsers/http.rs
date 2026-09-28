use syn::{
    Expr, Ident, LitInt, Result, Token,
    parse::{Parse, ParseStream},
};

pub(crate) struct HttpArgs {
    pub(crate) protos: Option<Expr>,
    pub(crate) handler: Option<Expr>,
    pub(crate) from_crate: Option<Ident>,
    pub(crate) hostname: Option<Expr>,
    pub(crate) root_directory: Option<Expr>,
    pub(crate) workers: Option<LitInt>,
    pub(crate) logger_queue_size: Option<LitInt>,
    pub(crate) log: Option<Expr>,
    pub(crate) port: Option<Expr>,
    pub(crate) interface: Option<Expr>,
    pub(crate) tls: Option<Expr>,
    pub(crate) allow_unsafe_conn: Option<Expr>,
}

impl Parse for HttpArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut protos = None;
        let mut handler = None;
        let mut from_crate = None;
        let mut hostname = None;
        let mut root_directory = None;
        let mut workers = None;
        let mut logger_queue_size = None;
        let mut log = None;
        let mut port = None;
        let mut interface = None;
        let mut tls = None;
        let mut allow_unsafe_conn = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;

            match key
                .to_string()
                .as_str()
            {
                "protos" => {
                    if protos.is_some() {
                        return Err(input.error("Duplicate 'protos' key"));
                    }
                    let expr: Expr = input.parse()?;
                    protos = Some(expr);
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
                "logger_queue_size" => {
                    if logger_queue_size.is_some() {
                        return Err(input.error("Duplicate 'logger_queue_size' key"));
                    }
                    let expr: LitInt = input.parse()?;
                    logger_queue_size = Some(expr);
                }
                "workers" => {
                    if workers.is_some() {
                        return Err(input.error("Duplicate 'workers' key"));
                    }
                    let expr: LitInt = input.parse()?;
                    workers = Some(expr);
                }
                "log" => {
                    if log.is_some() {
                        return Err(input.error("Duplicate 'log' key"));
                    }
                    let expr: Expr = input.parse()?;
                    log = Some(expr);
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
                "tls" => {
                    if tls.is_some() {
                        return Err(input.error("Duplicate 'tls' key"));
                    }
                    let expr: Expr = input.parse()?;
                    tls = Some(expr);
                }
                "allow_unsafe_conn" => {
                    if allow_unsafe_conn.is_some() {
                        return Err(input.error("Duplicate 'allow_unsafe_conn' key"));
                    }
                    let expr: Expr = input.parse()?;
                    allow_unsafe_conn = Some(expr);
                }
                _ => return Err(input.error(format!("Unknown key: {}", key))),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(HttpArgs {
            protos,
            handler,
            from_crate,
            hostname,
            root_directory,
            workers,
            logger_queue_size,
            log,
            port,
            interface,
            tls,
            allow_unsafe_conn,
        })
    }
}
