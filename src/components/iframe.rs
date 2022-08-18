use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlIFrameElement};
use yew::{html, Component, Context, Properties};

const BOOTSTRAP_CSS_LINK: &str = "<link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC\" crossorigin=\"anonymous\">";

pub struct IFrame {}

#[derive(Eq, PartialEq, Properties)]
pub struct IFrameProperties {
    pub content: String,
    #[prop_or(10)]
    pub margin: i32,
    #[prop_or(true)]
    pub include_style: bool,
}

pub enum Msg {
    Resize(Event),
}

impl Component for IFrame {
    type Message = Msg;
    type Properties = IFrameProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let srcdoc = if ctx.props().include_style {
            format!("{}\n\n{}", BOOTSTRAP_CSS_LINK, ctx.props().content)
        } else {
            ctx.props().content.clone()
        };

        html! {
            <iframe {srcdoc} onload={ctx.link().callback(Msg::Resize)} width="100%"></iframe>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Resize(event) => {
                let target = event.target().unwrap_throw();
                let iframe: HtmlIFrameElement = target.dyn_into().unwrap_throw();
                let mut height = iframe
                    .content_window()
                    .unwrap_throw()
                    .document()
                    .unwrap_throw()
                    .body()
                    .unwrap_throw()
                    .scroll_height();
                height += ctx.props().margin;
                iframe.set_height(&format!("{height}px"));
                true
            }
        }
    }
}
