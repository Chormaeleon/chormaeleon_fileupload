use yew::{function_component, Html, html};

#[function_component(LoadingSpinner)]
pub fn loading_spinner() -> Html {
    html! {
        <>
            { "Lade..." }
            <div class="spinner-border text-danger" style="width: 1rem; height: 1rem;" role="status">
                <span class="visually-hidden">{ "Lade-Animation" } </span>
            </div>
        </>
    }
}