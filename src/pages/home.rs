use crate::{
    components::contribution_list::*,
    utilities::requests::fetch::{get_request_struct, FetchError},
};
use gloo_console::error;
use gloo_dialogs::alert;
use yew::prelude::*;

pub struct Home {
    contributions: Option<Vec<ContributionListItem>>,
}

pub enum Msg {
    ContributionsLoaded(Vec<ContributionListItem>),
    ContributionsLoadError(FetchError),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        /*
        Self {
            contributions: Some( vec![
                    Contribution {
                        id: 0,
                        name: "Ode an die Freude".into(),
                        due: "22.08.2022".into(),
                    },
                    Contribution {
                        id: 1,
                        name: "So soll es bleiben".into(),
                        due: "10.03.2022".into(),
                    },
                ],
            ),
        }*/
        Self {
            contributions: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="container">
                <div class="row">
                    <div class="col">
                        <h2> { "Alle Abgaben" } </h2>
                        if let Some(contributions) = self.contributions.clone() {
                            <ContributionList contributions={contributions}/>
                        } else {
                            { "Abgaben werden geladen..." }
                        }
                    </div>
                </div>
            </div>

            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(async {
                match get_request_struct::<Vec<ContributionListItem>>(
                    "http://localhost:8001/pendingProjects".to_string(),
                )
                .await
                {
                    Ok(contributions) => Msg::ContributionsLoaded(contributions),
                    Err(error) => Msg::ContributionsLoadError(error),
                }
            })
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContributionsLoaded(contribs) => {
                self.contributions = Some(contribs);
                true
            }
            Msg::ContributionsLoadError(error) => {
                alert("Could not download list!");
                match error {
                    FetchError::JsError(js_error) => error!(js_error),
                    FetchError::WrongContentType => {
                        error!("Content type of fetched object was wrong!")
                    }
                    FetchError::SerdeError(serde_error) => {
                        error!("type could not be parsed!");
                        error!(serde_error.to_string());
                    }
                }
                true
            }
        }
    }
}
