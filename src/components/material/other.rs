use yew::{html, Context, Html};

use crate::{
    components::material::{AdminOrOwner, MaterialDeleteButton, MaterialUpdateButton},
    service::material::{material_url, MaterialTo},
};

use super::{DeleteMessage, Material, Msg, UpdateMessage};

pub fn other_list(ctx: &Context<Material>, other_elements: Vec<&MaterialTo>) -> Html {
    html! {
        <div class="row mt-2">
            <div class="col">
                <h2>{ "Sonstige Dateien + Downloads" }</h2>

                <table class="table table-striped">
                    <thead>
                        <tr>
                            <th>
                                { "Dateibeschreibung" }
                            </th>
                            <th>
                                { "Link" }
                            </th>
                            <AdminOrOwner owner_id={ ctx.props().project_owner }>
                                <th>
                                    { "Ändern" }
                                </th>
                                <th>
                                    { "Löschen" }
                                </th>
                            </AdminOrOwner>
                        </tr>
                    </thead>
                    <tbody>
                    {
                        for other_elements.iter().map(|other| {
                            other_element(ctx, (*other).clone())
                        })
                    }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

fn other_element(ctx: &Context<Material>, other: MaterialTo) -> Html {
    let other_clone = other.clone();
    html! {
        <tr>
            <td>
                { &other.title }
            </td>
            <td>
                <a href={ material_url(ctx.props().id, &other.file_technical_name) } download={ other.file_name.clone().to_string() }> { &other.file_name } </a>
            </td>
            <AdminOrOwner owner_id={ ctx.props().project_owner }>
                <td>
                    <MaterialUpdateButton
                        onclick={ ctx.link().callback(move |_| Msg::Update(UpdateMessage::ButtonClick(other_clone.clone()))) }
                        owner_id={ ctx.props().project_owner }
                    />
                </td>
                <td>
                    <MaterialDeleteButton
                        onclick={
                            ctx.link().callback(move |_|
                                Msg::Delete(DeleteMessage::DeleteButtonClick(other.clone()))
                            )
                        } 
                        owner_id={ ctx.props().project_owner }
                    />
                </td>
            </AdminOrOwner>
        </tr>
    }
}
