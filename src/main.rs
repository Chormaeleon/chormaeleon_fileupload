use gloo_console::{error, info, warn};
use gloo_dialogs::alert;
use service::{get_config, Config, CONFIG};
use utilities::requests::fetch::FetchError;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

mod components;

mod pages;
use pages::project::ProjectComponent;

use crate::components::jwt_context::JWTProvider;
use pages::home::Home;

mod service;
mod utilities;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/events/:id")]
    Event { id: i64 },
    #[at("/index.html")]
    Index,
}

pub enum AppWrapperMsg {
    LoadedConfig(Config),
    LoadConfigError(FetchError),
}

pub struct AppWrapper {
    content_loaded: bool,
}

impl Component for AppWrapper {
    type Message = AppWrapperMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content_loaded: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppWrapperMsg::LoadedConfig(config) => {
                info!(&format!("config loaded: {:#?}", config));
                match CONFIG.set(config.clone()) {
                    Ok(()) => {
                        info!("Set config")
                    }
                    Err(previous_config) => {
                        warn!(&format!(
                            "Overwriting config, old: {:#?}, new: {:#?}",
                            previous_config, config
                        ));
                    }
                }
                self.content_loaded = true;

                true
            }
            AppWrapperMsg::LoadConfigError(error) => {
                error!("Could not load config!");
                error!(error.to_string());
                alert("Fehler: Die Konfiguration konnte nicht geladen werden.");
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.content_loaded {
            info!("Rendering config loaded");
            html! {
                <App/>
            }
        } else {
            info!("Rendering config not loaded");
            html! {
                <div class="container">
                    <div class="row mt-2">
                        <div class="col text-center">
                            <h3>
                            { "Lade Einstellungen..." }
                            <div class="spinner-border text-danger" style="width: 3rem; height: 3rem;" role="status">
                            <span class="visually-hidden">{ "Lade-Animation" } </span>
                            </div>
                            </h3>
                        </div>
                    </div>
                </div>
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(async {
                let config = get_config().await;

                match config {
                    Ok(config) => AppWrapperMsg::LoadedConfig(config),
                    Err(error) => AppWrapperMsg::LoadConfigError(error),
                }
            });
        }
    }
}

pub enum Msg {}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <JWTProvider>
            <BrowserRouter>
                <main>
                        <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
            </JWTProvider>
        }
    }
}

#[allow(clippy::let_unit_value)]
fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! { <Home/> }
        }
        Route::Event { id } => {
            html! {
                 <div class="container mb-2">
                    <ProjectComponent id={ id }/>
                </div>
            }
        }
        Route::Index => {
            html! { <Home/> }
        }
    }
}

fn main() {
    yew::Renderer::<AppWrapper>::new().render();
}
