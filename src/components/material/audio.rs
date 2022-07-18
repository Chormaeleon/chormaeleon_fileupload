use yew::{html, Context, Html};

use crate::{
    components::material::MaterialDeleteButton,
    service::material::{material_url, MaterialTo},
};

use super::{DeleteMessage, Material, Msg};

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
    html! {
        <div class="col">
            <h5> { &audio.title } </h5>
            <audio controls=true id={ format!("audio-{}", audio.id)} src={ material_url(id, &audio.file_technical_name) }></audio>
            <h6> <i> { &audio.file_name } </i> </h6>
            <MaterialDeleteButton
                onclick={ ctx.link().callback(move |_| Msg::Delete(DeleteMessage::DeleteButtonClick(audio.clone()))) }
                owner_id={ ctx.props().project_owner }
            />
        </div>
    }
}
