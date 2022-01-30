use super::client_builder;
use plex_api::{Client, ClientBuilder};
use rstest::fixture;

#[fixture]
pub fn client_anonymous(client_builder: ClientBuilder) -> Client {
    client_builder
        .build()
        .expect("failed to create testing http client")
}

#[fixture]
pub fn client_authenticated(client_builder: ClientBuilder) -> Client {
    let token = std::env::var("PLEX_API_AUTH_TOKEN").unwrap_or_else(|_| "".to_owned());
    if token.is_empty() {
        panic!("PLEX_API_AUTH_TOKEN must be set!");
    }

    client_builder
        .set_x_plex_token(token)
        .build()
        .expect("failed to create testing http client")
}