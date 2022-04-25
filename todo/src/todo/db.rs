use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Database, error::Error};



#[derive(Clone)]
pub(crate) struct Conn(pub Database);


impl Conn{
    pub(crate) async fn new(uri: String) -> Result<Self,Error>{
        let options = ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;
        let client = Client::with_options(options)?;
        let db = client.database("todo");
        Ok(Self(db))
    }
}

