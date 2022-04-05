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
    pub multiple: bool,
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
                self.current_request = None;
                self.progress = None;
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
