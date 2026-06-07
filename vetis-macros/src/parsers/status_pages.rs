use syn::{
    parse::{Parse, ParseStream},
    Expr, LitInt, Result, Token,
};

pub(crate) struct StatusPagesArgs {
    pub(crate) pages: Vec<(LitInt, Expr)>,
}

impl Parse for StatusPagesArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut pages = Vec::new();
        while !input.is_empty() {
            let code: LitInt = input.parse()?;
            input.parse::<Token![@]>()?;
            let file: Expr = input.parse()?;
            pages.push((code, file));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(StatusPagesArgs { pages })
    }
}
