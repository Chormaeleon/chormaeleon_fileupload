mod progress;

use progress::Progress;

use yew::{html, Component, Properties};

pub enum ProgressMsg {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub loaded: f64,
    pub total: f64,
}

pub struct ProgressComponent {
    progress: Progress,
}

impl Component for ProgressComponent {
    type Message = ProgressMsg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let progress = Progress::new(ctx.props().loaded, ctx.props().total);
        Self { progress }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let percent = self.progress.loaded() / self.progress.total() * 100.0;
        html! {
            <>
            <div class="progress">
                <div class="progress-bar" role="progressbar" id="fileUploadProgress" style={ format!("width: {}%", percent)} aria-valuenow={self.progress.loaded().to_string()} aria-valuemin="0" aria-valuemax={self.progress.total().to_string()}></div>
            </div>
            <p>{ self.progress.loaded_string() } { self.progress.loaded_unit() } { " von " } { self.progress.total_string() } { self.progress.total_unit() } { " geladen" }</p>
            </>
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        self.progress.set_loaded(ctx.props().loaded);
        self.progress.set_total(ctx.props().total);
        true
    }
}
