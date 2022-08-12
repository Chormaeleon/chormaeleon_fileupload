use gloo_dialogs::alert;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{ErrorEvent, ProgressEvent};
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::components::progress::ProgressComponent;

use xmlhttp::xmlhttp_post_request::{PostRequest, SentRequest};

pub enum Msg {
    Files,
    UploadFile,
    UploadUpdate { loaded: f64, total: f64 },
    UploadOnload,
    UploadOnerror(String),
    Abort,
}

struct Progress {
    loaded: f64,
    total: f64,
}

pub struct Upload {
    progress: Option<Progress>,
    upload_successfully_finished: bool,
    current_request: Option<SentRequest>,
}

#[derive(PartialEq, Properties)]
pub struct UploadProperties {
    pub form_id: String,
    pub field_name: String,
    pub target_url: String,
    pub multiple: bool,
    pub success_callback: Callback<String>,
    pub failure_callback: Callback<String>,
}

impl Component for Upload {
    type Message = Msg;
    type Properties = UploadProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            progress: None,
            upload_successfully_finished: false,
            current_request: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files => {
                ctx.link().send_message(Msg::UploadFile);
                false
            }
            Msg::UploadFile => {
                self.upload_successfully_finished = false;
                let mut request = PostRequest::new_from_form(&ctx.props().form_id);

                let link = ctx.link().to_owned();

                let progress_callback = move |progress_event: ProgressEvent| {
                    link.send_message(Msg::UploadUpdate {
                        loaded: progress_event.loaded(),
                        total: progress_event.total(),
                    });
                };

                let link = ctx.link().to_owned();

                let error_callback = move |_error: ErrorEvent| {
                    link.send_message(Msg::UploadOnerror(
                        /*error.message() -> is undefined, creates a type error!*/
                        "Noch nicht verfügbar, siehe Konsole (F11)".to_string(),
                    ));
                };

                request.set_upload_onprogress(Some(Box::new(progress_callback)));
                request.set_upload_onerror(Some(Box::new(error_callback)));

                let link = ctx.link().to_owned();

                let finish_callback = move |_| link.send_message(Msg::UploadOnload);

                request.set_request_onload(Some(Box::new(finish_callback)));

                let sent_request = request
                    .send(&ctx.props().target_url)
                    .expect("Could not send! Error happened!");

                self.current_request = Some(sent_request);

                true
            }
            Msg::UploadOnerror(message) => {
                alert(&format!("Beim Upload ist ein Fehler aufgetreten! Bitte versuche es erneut und wende dich dann an den/die Administrator*in. Fehlermeldung: {}", message));
                self.upload_successfully_finished = false;
                self.progress = None;
                self.current_request = None;
                true
            }
            Msg::UploadUpdate { loaded, total } => {
                match &mut self.progress {
                    Some(progress) => {
                        progress.loaded = loaded;
                        progress.total = total;
                    }
                    None => {
                        self.progress = Some(Progress { loaded, total });
                    }
                }
                true
            }
            Msg::UploadOnload => {
                let request = self.current_request.take().unwrap().request;
                self.progress = None;
                let text = request.response_text().unwrap().unwrap();

                match request.status().unwrap_throw() {
                    200 | 201 | 304 => {
                        self.upload_successfully_finished = true;
                        ctx.props().success_callback.emit(text);
                    }
                    _ => {
                        self.upload_successfully_finished = false;
                        ctx.props().failure_callback.emit(text);
                    }
                }

                true
            }
            Msg::Abort => {
                let current_request = self.current_request.take();

                if let Some(request) = current_request {
                    request.abort();
                    self.progress = None;
                    self.current_request = None;
                    return true;
                }

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div>

                    <input type="file" class="form-control" name={ctx.props().field_name.clone()} multiple={ ctx.props().multiple }/>

                    if let Some(progress) = &self.progress {
                        <div class="mt-2">
                            <h4>{ "Upload läuft" }</h4>
                            <ProgressComponent loaded={ progress.loaded } total={ progress.total }/>
                        </div>
                    }
                    <div class="row mt-2">
                        <div class="col">
                            if self.upload_successfully_finished {
                                <div class="mt-2">
                                <h4> { "Upload erfolgreich!" } </h4>
                                </div>
                            }
                            </div>
                        <div class="col text-end">
                            if self.current_request.is_some() {
                                <button type="button" class="btn btn-danger" onclick={ctx.link().callback(move|_| { Msg::Abort })}> { "Abbrechen" } </button>
                            } else {
                            <button type="button" class="btn btn-danger" onclick={ctx.link().callback(move |_| { Msg::Files })}> { "Upload starten" }</button>
                            }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
