# Turnin Frontend
This is the choir turnin frontend application.

## Build
You can build the application using [trunk](https://github.com/thedodd/trunk). Trunk can be installed using `cargo install trunk`.
We also need to install the wasm target `rustup target add wasm32-unknown-unknown`.

Then, you can run `trunk build --release` to build or `trunk serve --release` for testing on port 8080.

### Configuration
To locate the right backend and authentication server, the app fetches a config file at runtime.

By default, the app uses a development configuration sourced from `config_local.json`. To override that for release, create a file `config_deploy.json`. If `config_deploy.json` is found, `config_local.json` is ignored. More info about the values is located in service/mod.rs in the config struct that the configration is parsed into.
