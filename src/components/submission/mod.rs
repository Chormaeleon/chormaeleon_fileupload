pub mod details;
pub mod list;
pub mod update;

use yew::{function_component, html, Callback, Properties};

use crate::{
    service::submission::{Section, SubmissionKind},
    utilities::callback::{
        convert_string_callback, select_to_enum_callback_fallible,
        select_to_enum_callback_infallible,
    },
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct InputSubmissionNoteProperties {
    #[prop_or_default]
    pub on_input: Callback<String>,
    #[prop_or_default]
    pub value: Option<String>,
    pub id: String,
}

#[function_component(InputSubmissionNote)]
pub fn input_submission_note(props: &InputSubmissionNoteProperties) -> Html {
    html! {
        <>
        <label for={ props.id.clone() }> { "Anmerkungen" } </label>
        <input id={ props.id.clone() }
            type="text"
            class="form-control"
            name="note"
            maxlength="100"
            value={ props.value.clone() }
            placeholder="z.B. Takt 15 bitte rausschneiden..."
            oninput={convert_string_callback(props.on_input.clone())}/>
        </>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct InputSubmissionSectionProperties {
    #[prop_or_default]
    pub on_input: Callback<Result<Section, ()>>,
    pub selected: Section,
    pub id: String,
}

#[function_component(InputSubmissionSection)]
pub fn input_submission_section(props: &InputSubmissionSectionProperties) -> Html {
    let section = props.selected;
    html! {
        <>
        <label for={ props.id.clone() }> { "Stimme" } </label>
        <select id={ props.id.clone() } name="section" class="form-control" required=true oninput={select_to_enum_callback_fallible(props.on_input.clone())}>
            <option value="Soprano" selected={ section == Section::Soprano }> { "Sopran" }</option>
            <option value="Alto" selected={ section == Section::Alto }> { "Alt" }</option>
            <option value="Tenor" selected={ section == Section::Tenor }> { "Tenor" }</option>
            <option value="Bass" selected={ section == Section::Bass }> { "Bass" }</option>
            <option value="Conductor" selected={ section == Section::Conductor }> { "Dirigent" }</option>
            <option value="Instrument" selected={ section == Section::Instrument }> { "Instrument" }</option>
        </select>
        </>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct InputSubmissionKindProperties {
    #[prop_or_default]
    pub on_input: Callback<SubmissionKind>,
    pub selected: SubmissionKind,
    pub id: String,
}

#[function_component(InputSubmissionKind)]
pub fn input_submission_kind(props: &InputSubmissionKindProperties) -> Html {
    let selection = props.selected;
    html! {
        <>
        <label for={ props.id.clone() }> { "Art" } </label>
        <select id={ props.id.clone() } name="kind" class="form-control" required=true oninput={ select_to_enum_callback_infallible(props.on_input.clone()) }>
            <option value="video" selected={ selection == SubmissionKind::Video} >{ "Video" }</option>
            <option value="audio" selected={ selection == SubmissionKind::Audio}>{ "Audio" }</option>
            <option value="other" selected={ selection == SubmissionKind::Other}>{ "Sonstiges" }</option>
        </select>
        </>
    }
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct InputSubmissionCreatorNameProperties {
    #[prop_or_default]
    pub on_input: Callback<String>,
    #[prop_or_default]
    pub value: Option<String>,
    pub id: String,
}

#[function_component(InputSubmissionCreatorName)]
pub fn input_submission_creator_name(props: &InputSubmissionCreatorNameProperties) -> Html {
    html! {
        <>
        <label for={ props.id.clone() }> { "Ersteller*in" } </label>
        <input id={ props.id.clone() }
            type="text"
            class="form-control"
            name="creator_name"
            maxlength="100"
            value={ props.value.clone() }
            placeholder={ "Name des/der Ersteller*in der Abgabe" }
            oninput={convert_string_callback(props.on_input.clone())}/>
        </>
    }
}