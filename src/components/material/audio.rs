use yew::{html, Html};

use crate::service::material::{material_url, MaterialTo};

pub fn audio_list(id: i64, audio_elements: Vec<&MaterialTo>) -> Html {
    html! {
        <>
        <div class="row mt-2">
            <div class="col">
                <h4>{ "Playbacks" }</h4>
            </div>
        </div>
        <div class="row">
        {
            if audio_elements.is_empty() {
                html!{
                    <p>{ "Noch keine Audiodateien gefunden!" }</p>
                }
            } else {
                html! {
                    for audio_elements.iter().map(move |audio|  {
                        audio_element(id, (*audio).clone())
                    })
                }
            }
        }
        </div>
        </>
    }
}

fn audio_element(id: i64, audio: MaterialTo) -> Html {
    html! {
        <div class="col">
            <h5> { &audio.title } </h5>
            <audio controls=true id={ format!("audio-{}", audio.id)} src={ material_url(id, &audio.file_technical_name) }></audio>
            <h6> <i> { &audio.file_name } </i> </h6>
        </div>
    }
}
