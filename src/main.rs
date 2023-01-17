use std::env;
use std::process::exit;

use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::http_client;
use openidconnect::{
    ClientId, ClientSecret, IssuerUrl, OAuth2TokenResponse, ResourceOwnerPassword,
    ResourceOwnerUsername,
};

use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest;
use std::error::Error;

// Helper function to help printing errors in a consistent way
fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) {
    let mut err_msg = format!("ERROR: {}", msg);
    let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
    while let Some(cause) = cur_fail {
        err_msg += &format!("\n    caused by: {}", cause);
        cur_fail = cause.source();
    }
    println!("{}", err_msg);
    exit(1);
}

// Uses the OpenID Connect library to get an access token from the IdP
fn get_access_token() -> String {
    // Get the env variables as needed
    let client_id = ClientId::new(env::var("CLIENT_ID").expect("set CLIENT_ID"));
    let client_secret = ClientSecret::new(env::var("CLIENT_SECRET").expect("set CLIENT_SECRET"));
    let username = ResourceOwnerUsername::new(env::var("USERNAME").expect("set USERNAME"));
    let password = ResourceOwnerPassword::new(env::var("PASSWORD").expect("set PASSWORD"));
    let oidc_url = env::var("OIDC_URL").expect("set OIDC_URL");

    // setup the provider from the discovery URL
    let issuer_url = IssuerUrl::new(oidc_url.to_string()).expect("Invalid issuer URL");
    let provider_metadata = CoreProviderMetadata::discover(&issuer_url, http_client)
        .unwrap_or_else(|err| {
            handle_error(&err, "Failed to discover OpenID Provider");
            unreachable!();
        });

    // Set up the config for the OAuth2 process.
    let client =
        CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret));

    // Exchange the code with the password (requires Direct Access Grant enabled).
    // Not using the standard to avoid browser interaction.
    // Is considered safe as the client is not public.
    let token_response = client
        .exchange_password(&username, &password)
        .request(http_client)
        .unwrap_or_else(|err| {
            handle_error(&err, "Failed to contact token endpoint");
            unreachable!();
        });

    //println!("returned scopes: {:?}", token_response.scopes());

    token_response.access_token().secret().to_string()
}

// Define a Graphql Query - Me Query
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql", // partial schema
    query_path = "src/me_query.graphql", // this query definition
    response_derives = "Debug"
)]
pub struct MeQuery;

// Executes the query and returns the type (type is defined automatically and is type safe
// with the struc definition
fn perform_me_query(
    client: reqwest::blocking::Client,
) -> Result<me_query::MeQueryMeMe, Box<dyn Error>> {
    // Get the API_URL from the environment
    let api_url = env::var("API_URL").expect("set API_URL");

    // Build the query and execute
    let variables = me_query::Variables {}; // Using empty variables
    let res = post_graphql::<MeQuery, _>(&client, api_url, variables)?;
    let me_response: me_query::ResponseData = res.data.expect("No data in response");

    // iterate over me_response.me.errors and print them
    for error in me_response.me.errors {
        println!("Error: {:?}", error);
    }

    // Return only the me part of the response
    Ok(me_response.me.me.expect("Error found"))
}

// Builds a sync client with the access token in the Authorization header
fn build_client(access_token: String) -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()
        .unwrap()
}

fn main() {
    // Get the access token
    let access_token = get_access_token();

    // Build the client
    let client = build_client(access_token);

    // Perform the query
    let me = perform_me_query(client).unwrap();

    // Print the result
    println!("my id: {}", me.id.to_string());
    println!("my email: {}", me.email);
    println!("my username: {}", me.username.expect("Missing Username"));
}
