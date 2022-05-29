use serde::Deserialize;
use yew::{classes, function_component, html, Properties};
use yew_router::prelude::Link;

use crate::Route;

#[derive(Default, Deserialize, PartialEq, Clone)]
pub struct ContributionListItem {
    pub id: i32,
    pub title: String,
    pub due: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Contributions {
    pub(crate) contributions: Vec<ContributionListItem>,
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
                                    { &upload.title }
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
