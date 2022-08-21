use yew::{html, Html};

use crate::service::material::{material_url, MaterialTo};

pub fn audio_list(id: i32, audio_elements: Vec<&MaterialTo>) -> Html {
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
                audio_element(id, (*audio).clone())
            })
        }
        </div>
        </>
    }
}

fn audio_element(id: i32, audio: MaterialTo) -> Html {
    html! {
        <div class="col">
            <h5> { &audio.title } </h5>
            <audio controls=true id={ format!("audio-{}", audio.id)} src={ material_url(id, &audio.file_technical_name) }></audio>
            <h6> <i> { &audio.file_name } </i> </h6>
        </div>
    }
}
