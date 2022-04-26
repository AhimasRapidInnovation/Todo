use std::ops::Deref;

use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    Client, Database,
};

#[derive(Clone, Debug)]
pub(crate) struct Conn(pub Database);

impl Conn {
    pub(crate) async fn new(uri: String) -> Result<Self, Error> {
        let options =
            ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;
        let client = Client::with_options(options)?;
        let db = client.database("todo");
        Ok(Self(db))
    }
}

impl Deref for Conn {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
