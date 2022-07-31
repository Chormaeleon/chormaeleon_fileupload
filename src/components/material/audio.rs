use yew::{html, Context, Html};

use crate::{
    components::material::{MaterialDeleteButton, MaterialUpdateButton},
    service::material::{material_url, MaterialTo},
};

use super::{DeleteMessage, Material, Msg, UpdateMessage};

pub fn audio_list(ctx: &Context<Material>, id: i32, audio_elements: Vec<&MaterialTo>) -> Html {
    html! {
        <>
        <div class="row mt-2">
            <div class="col">
                <h2>{ "Playbacks" }</h2>
            </div>
        </div>
        <div class="row">
        {
            for audio_elements.iter().map(move |audio|  {
                audio_element(ctx, id, (*audio).clone())
            })
        }
        </div>
        </>
    }
}

fn audio_element(ctx: &Context<Material>, id: i32, audio: MaterialTo) -> Html {
    let audio_clone = audio.clone();
    html! {
        <div class="col">
            <h5> { &audio.title } </h5>
            <audio controls=true id={ format!("audio-{}", audio.id)} src={ material_url(id, &audio.file_technical_name) }></audio>
            <h6> <i> { &audio.file_name } </i> </h6>
            <div class="row">
                <div class="col-auto">
                    <MaterialUpdateButton
                        onclick={ ctx.link().callback(move |_| Msg::Update(UpdateMessage::ButtonClick(audio_clone.clone()))) }
                        owner_id={ ctx.props().project_owner }
                    />
                </div>
                <div class="col">
                    <MaterialDeleteButton
                        onclick={ ctx.link().callback(move |_| Msg::Delete(DeleteMessage::DeleteButtonClick(audio.clone()))) }
                        owner_id={ ctx.props().project_owner }
                    />
                </div>
            </div>
        </div>
    }
}
