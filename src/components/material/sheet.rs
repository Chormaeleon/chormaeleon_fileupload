use yew::{html, Context, Html};

use crate::{
    components::material::{MaterialDeleteButton, MaterialUpdateButton},
    service::material::{material_url, MaterialTo},
};

use super::{DeleteMessage, Material, Msg, UpdateMessage};

pub fn sheet_list(ctx: &Context<Material>, id: i32, sheet_elements: Vec<&MaterialTo>) -> Html {
    html! {
        <div class="row mt-2">
            <div class="col">
                <h2>{ "Noten" }</h2>
                {
                    for sheet_elements.iter().map(|score| {
                        sheet_element(ctx, id, (*score).clone())
                    })
                }
            </div>
        </div>
    }
}

fn sheet_element(ctx: &Context<Material>, id: i32, score: MaterialTo) -> Html {
    let score_clone = score.clone();
    html! {
        <>
        <div class="row">
            <div class="col">
                <h5> { &score.title } </h5>
                <div class="mt-2 ratio ratio-16x9">
                    <embed src={ format!("{}#toolbar=0&navpanes=0&scrollbar=0&statusbar=0&messages=0", material_url(id, &score.file_technical_name)) }/>
                </div>
                <h6> <i> { &score.file_name } </i> </h6>
            </div>
        </div>
        <div class="row">
            <div class="col-auto">
                <MaterialUpdateButton
                        onclick={ ctx.link().callback(move |_| Msg::Update(UpdateMessage::ButtonClick(score_clone.clone()))) }
                        owner_id={ ctx.props().project_owner }
                />
            </div>
            <div class="col">
                <MaterialDeleteButton
                    onclick={
                        ctx.link().callback(move |_|
                            Msg::Delete(DeleteMessage::DeleteButtonClick(score.clone()))
                        )
                    }
                    owner_id={ ctx.props().project_owner }
                />
            </div>
        </div>
        </>
    }
}
