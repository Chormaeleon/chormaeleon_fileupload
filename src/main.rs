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
    Event { id: i32 },
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
                        <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
            </JWTProvider>
        }
    }
}

#[allow(clippy::let_unit_value)]
fn switch(routes: &Route) -> Html {
    match routes.clone() {
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
    }
}

fn main() {
    yew::start_app::<App>();
}
