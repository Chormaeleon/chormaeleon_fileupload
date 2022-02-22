use serde::Deserialize;
use yew::{function_component, html, Properties, classes};
use yew_router::prelude::Link;

use crate::Route;

#[derive(Deserialize, PartialEq, Clone)]
pub struct ContributionListItem {
    pub id: usize,
    pub name: String,
    pub due: String,
}


#[derive(Properties, PartialEq, Clone)]
pub struct Contributions {
    pub(crate) contributions: Vec<ContributionListItem>
}

#[function_component(ContributionList)]
pub fn contribution_list(contributions: &Contributions) -> Html {
    html! {
        <table class="table table-striped">
            <thead>
                <tr>
                    <th>
                        { "Stück" }
                    </th>
                    <th>
                        { "Abgabe bis" }
                    </th>
                </tr>
                </thead>
                    <tbody>
                    { for contributions.contributions.iter().map(|upload| html!{
                        <tr>
                            <td>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Event{id: upload.id}}>
                                    { &upload.name }
                                </Link<Route>>
                            </td>
                            <td>
                                { &upload.due }
                            </td>
                        </tr>
                    }) }
                    </tbody>
            </table>
    }
}
