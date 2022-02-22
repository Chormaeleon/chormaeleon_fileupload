use gloo_dialogs::alert;
use web_sys::ProgressEvent;
use yew::{html, Component, Context, Html, Properties};

use crate::components::progress::ProgressComponent;

use crate::utilities::requests::xmlhttp_post_request::{PostRequest, SentRequest};

pub enum Msg {
    Files,
    UploadFile,
    UploadUpdate { loaded: f64, total: f64 },
    UploadOnload,
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

                request.set_header("Authorization".to_string(), "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzZWN0aW9uIjoiU29wcmFuIiwibmFtZSI6Ik1heGkgTXVzdGVybWFubiIsImlzX2FkbWluIjpmYWxzZSwidXNlcl9pZCI6MSwiZXhwIjoyNjAzMTM0MjA1fQ.MWejEAktVwjgmHztxd7QOx3TMG8Ek2jLr3AbVwBj0nfpvN6wQxuggUXoy9MReTvjpEDh0QSgC1ElHJymtnrKEA".to_string());

                request.set_upload_onprogress(Some(Box::new(progress_callback)));

                let link = ctx.link().to_owned();

                let finish_callback = move |_| link.send_message(Msg::UploadOnload);

                request.set_request_onload(Some(Box::new(finish_callback)));

                let sent_request = request
                    .send(&ctx.props().target_url)
                    .expect("Could not send! Error happened!");

                self.current_request = Some(sent_request);

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
                //self.current_request = None;
                self.upload_successfully_finished = true;
                self.progress = None;
                true
            }
            Msg::Abort => {
                let current_request = self.current_request.take();

                match current_request {
                    Some(request) => {
                        request.abort();
                        self.progress = None;
                    }
                    None => (),
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div>
                    <p>{ "Hier könnt ihr eure Datei hochladen" }</p>
                    <form id={ctx.props().form_id.clone()} class="" name="form2" enctype="multipart/form-data">
                    <input type="file" class="form-control" name={ctx.props().field_name.clone()} multiple=true onchange={ctx.link().callback(move |_| {
                            Msg::Files
                        })}
                    />
                    </form>
                    if let Some(progress) = &self.progress {
                        <div class="mt-2">
                        <h4>{ "Upload läuft" }</h4>
                        <ProgressComponent loaded={ progress.loaded } total={ progress.total }/>
                        </div>
                    }
                    if let Some(request) = &self.current_request {
                        <div class="mt-2">
                            <button type="button" class="btn btn-danger" onclick={ctx.link().callback(move|_| { Msg::Abort })}> { "Abbrechen" } </button>
                            <p>{ request.ready_state().to_string() }</p>
                        </div>
                    }
                    if self.upload_successfully_finished {
                        <div class="mt-2">
                        <h4> { "Upload erfolgreich!" } </h4>
                        </div>
                    }
                </div>
            </div>
        }
    }
}
