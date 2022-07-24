# Building on the twitter API with Rust

The examples int this repository use the [`twitter_v2`](https://github.com/jpopesculian/twitter-v2-rs) crate to interface with the twitter API, feel free to visit the repo and give it a star.

## getting started
Install [rust](https://www.rust-lang.org/tools/install) to your machine.

Fork the repo (I'm assuming you also have `git` installed)

```sh
git clone https://github.com/collinsmuriuki/birdie.git

cd birdie
```

Copy the contents in `.env.example` to `.env`

```
cp .env.example .env
```

To make sure everything works run:

```sh
cargo check
```

## api keys and secrets

- Ensure you have a [twitter developer](https://developer.twitter.com) account and you have created a project and an application under the development environment
- On creating an application you will be immediately provided with an `API Key`, `API Key Secret` and `Bearer Token`: copy these into your `.env` file
- Next you want to enable `OAuth` in the `User authentication settings` under your application's settings tab
- Select both `OAuth1` and `Oauth2` and enable read and write access
- Add a callback url - use `http://127.0.0.1:8080/callback` and save
- On saving, you will receive a `Client ID` and `Client Secret` - copy these into your `.env` file
- Finally, in the settings tab, navigate to the `Keys and Tokens` tab and click `Regenerate` under `Access Token and Secret` - copy these values to your `.env` file as well

### navigating this repo

This repository is a cargo workspace organized into two projects
- [auth](./auth)
- [twitter_api](./twitter_api/)

### auth
- Here we have code related to authentication to the twitter api to generate access tokens via oauth2.0. There is also a binary containing a code example showing how you can use your [bearer token](./auth/src/bin/bearer_token.rs) to interact with the API
- In [`./auth/src/bin/oauth2_server.rs`](./auth/src/bin/oauth2_server.rs) you'll find a web server that demonstrates how you can create an authentication url and handle the twitter callback request.
- The `auth` project also exposes a library [`birdie_auth`](./auth/src/lib.rs) which contains a single `generate_oath1_token` function that does as the name suggests, generates an oauth1 token: this function is reused in the [`twitter_api`](./twitter_api) project to prevent code repetition
- How to run

```sh
# run bearer_token bin
cargo run --bin bearer_token
# run oauth1 bin
cargo run --bin oauth1
# run oauth2 server
cargo run --bin oath2_server
```

### twitter_api
- This project contains 3 submodules that contains abstractions over the [`lists_api`](./twitter_api/src/lists_api.rs) and [`tweet_api`](./twitter_api/src/tweet_api.rs)
- We only have a single binary in [`main.rs`](./twitter_api/src/main.rs) where we'll be running out examples

```sh
cargo run --bin twitter_api 
```

*Author - [collinsmuriuki](https://collinsmuriuki.xyz)*

*License: [MIT](LICENSE)*