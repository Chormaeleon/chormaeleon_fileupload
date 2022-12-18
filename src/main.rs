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
    yew::Renderer::<App>::new().render();
}
