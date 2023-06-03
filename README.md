# Turnin Frontend
This is the choir turnin frontend application.

## Build
You can build the application using [trunk](https://github.com/thedodd/trunk). Trunk can be installed using `cargo install`, but as of 0.16.0, we need the master branch for JS files work, you can do that via `cargo install trunk --force --git https://github.com/thedodd/trunk.git --branch master`

### Configuration
To locate the right backend and authentication server, the app fetches a config file at runtime.

By default, the app uses a development configuration sourced from `config_local.json`. To override that for release, create a file `config_deploy.json`. If `config_deploy.json` is found, `config_local.json` is ignored. More info about the values is located in service/mod.rs in the config struct that the configration is parsed into.
