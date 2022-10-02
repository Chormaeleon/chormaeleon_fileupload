use yew::{html, Context, Html};

use crate::service::material::{material_url, MaterialTo};

use super::Material;

pub fn video_list(ctx: &Context<Material>, video_elements: Vec<&MaterialTo>) -> Html {
    html! {
        <div class="row mt-2">
            <div class="col">
                <h4>{ "Videos" }</h4>
                {
                    if video_elements.is_empty() {
                        html!{
                            <p>{ "Noch keine Videodateien gefunden!" }</p>
                        }
                    } else {
                        html! {
                            for video_elements.iter().map(|video| {
                                video_element(ctx, (*video).clone())
                            })
                        }
                    }
                }
            </div>
        </div>
    }
}

fn video_element(ctx: &Context<Material>, video_element: MaterialTo) -> Html {
    html! {
        <div class="row">
            <div class="col">
            <h5> { &video_element.title } </h5>
            <div class="ratio ratio-16x9">
                    <video id={video_element.title.clone()} controls=true>
                        <source src={ material_url(ctx.props().id, &video_element.file_technical_name) }/>
                    </video>
                </div>
                <h6> <i> { &video_element.file_name } </i> </h6>
            </div>
        </div>
    }
}
