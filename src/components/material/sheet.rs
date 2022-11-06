use yew::{html, Html};

use crate::service::material::{material_url, MaterialTo};

pub fn sheet_list(id: i64, sheet_elements: Vec<&MaterialTo>) -> Html {
    html! {
        <div class="row mt-2">
            <div class="col">
                <h4>{ "Noten" }</h4>
                {
                    if sheet_elements.is_empty() {
                        html!{
                            <p>{ "Noch keine Noten gefunden!" }</p>
                        }
                    } else {
                        html! {
                        {
                            for sheet_elements.iter().map(|score| {
                                sheet_element(id, (*score).clone())
                            })
                        }
                    }
                    }
                }
            </div>
        </div>
    }
}

fn sheet_element(id: i64, score: MaterialTo) -> Html {
    html! {
        <div class="row">
            <div class="col">
                <h5> { &score.title } </h5>
                <div class="mt-2 ratio ratio-16x9">
                    <embed src={ format!("{}#toolbar=0&navpanes=0&scrollbar=0&statusbar=0&messages=0", material_url(id, &score.file_technical_name)) }/>
                </div>
                <h6> <i> { &score.file_name } </i> </h6>
            </div>
        </div>
    }
}
