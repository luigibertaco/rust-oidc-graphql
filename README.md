# rust-oidc-graphql
Playing with Rust and GraphQL for personal tests and research

## Clone

```shell
git clone https://github.com/luigibertaco/rust-oidc-graphql
cd rust-oidc-graphql
```

## How to test

1. Ensure rust is installed

1. Set required env variables
  - `CLIENT_ID`: OAuth2 client id
  - `CLIENT_SECRET`: OAuth2 client secret
  - `USERNAME`: Existing Oauth2 user username
  - `PASSWORD`: Existing Oauth2 user password
  - `OIDC_URL`: IdP server URL
  - `API_URL`: GraphQL API server URL

1. Execute:

    ```shell
    cargo run
    ```
1. Or from a single line:

    ```shell
    CLIENT_ID=... CLIENT_SECRET=... USERNAME=... PASSWORD=... API_URL=... OIDC_URL=... cargo run
    ```

## Ideas

- [x] Basic Oauth2 direct grant flow to retrieve access token
- [x] Basic GraphQL query execution with partial schema definition
- [ ] Access token expiring and caching strategy
- [ ] Custom directives and scalars usage
