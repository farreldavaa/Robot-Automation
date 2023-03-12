use yew::prelude::*;

use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use yew_router::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
// use serde::{
//     Deserialize,
//     Serialize,
// };
use crate::router::route::AppRoute;
use crate::types::var::{
    UserAccount,
};


// #[derive(Serialize, Debug, Clone)]
// pub struct UserAccount {
//     username: String,
//     status: String,
// }

pub enum Msg {
    InputText(String),
    InputSelect(String),
    Login,
    GetData(String),
    // SelectProject,
    Ignore,
}

pub struct PageInput {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component

    // DATA
    username: String,
    status: String,

    // SERVICES
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

impl Component for PageInput {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            // DATA
            username: String::from(""),
            status: String::from(""),

            // SERVICES
            link: link.clone(),
            fetch_task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputText(data) => {
                ConsoleService::info(&format!("data input is {:?}", data));
                // let test = data.to_owned();
                self.username = data;
                true
            }
            Msg::InputSelect(data) => {
                ConsoleService::info(&format!("data input select is {:?}", data));
                self.status = data;
                true
            }
            Msg::Login => {
                // FETCHING....

                let user_account = UserAccount {
                    username: self.username.clone(),
                    status: self.status.clone(),
                };


                let request = Request::post("http://localhost:3000/attack")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .header("Content-Type", "application/json")
                    .body(Json(&user_account))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("status is {:?}", status_number));

                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::GetData(dataok)
                            }
                            Err(error) => {
                                ConsoleService::info("ignore.");
                                Msg::Ignore
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);


                true
            }
            Msg::GetData(data) => {
                ConsoleService::info(&format!("get data {:?}", data));

                // self.router_agent.send(ChangeRoute(AppRoute::Other.into()));
                
                true
            }

            // Msg::SelectProject(data) =>{
            //     ConsoleService::info(&format!("get data {:?}",data));

            //     self.router_agent.send(ChangeRoute(AppRoute::Project.into()));
            //     true
            // }
            Msg::Ignore => {
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
        <div class="base-form">
            <div class="form">
            <div class="img-setting"><img src="img/test.png" alt="rust image" width="100" height="100"/></div>
                <h5>{"Basic Information"}</h5>
                <div class="input-group mb-3" style=" margin: auto; width: 400px;">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Name"
                        oninput=self.link.callback(|data: InputData| Msg::InputText(data.value))
                        />
                </div>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Description"
                        oninput=self.link.callback(|data: InputData| Msg::InputText(data.value))
                        />
                </div>
                <h5>{"Credential Platform"}</h5>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Email"
                        oninput=self.link.callback(|data: InputData| Msg::InputText(data.value))
                        />
                </div>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="API Key"
                        oninput=self.link.callback(|data: InputData| Msg::InputText(data.value))
                        />
                </div>
                <select class="form-select mb-4" style=" margin: auto; width: 400px;" aria-label="Default select example"
                    onchange=self.link.callback(|e| {
                        if let ChangeData::Select(select) = e {
                            let value = select.value();
                            Msg::InputSelect(value)
                        } else {
                            Msg::InputSelect("No value".to_string())
                        }
                    })
                >
                    <option>{ "Select Platform"}</option>
                    <option value="superhero">{ "Jira" }</option>
                    <option value="normal">{ "Telkom Automation" }</option>
                </select>
                 // onchange=self.link.callback(|_| {
                //     // Msg::SelectProject
                // })
                    <div>
                        <button type="button" class="btn btn-primary mb-3" data-bs-toggle="modal" data-bs-target="#exampleModal">
                            {"Project Selection"}
                        </button>
                    </div>
                <h5>{"Filter Setting"}</h5>
                <select class="form-select mb-4" style=" margin: auto; width: 400px;" aria-label="Default select example"
                    onchange=self.link.callback(|e| {
                        if let ChangeData::Select(select) = e {
                            let value = select.value();
                            Msg::InputSelect(value)
                        } else {
                            Msg::InputSelect("No value".to_string())
                        }
                    })
                >
                    <option>{ "Scheduler"}</option>
                    <option value="normal">{ "3 Days" }</option>
                    <option value="normal">{ "1 Week" }</option>
                    <option value="normal">{ "1 Month" }</option>
                </select>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                    <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                        <label class="form-check-label" for="flexCheckDefault">{"Double Email"}</label>
                </div>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                    <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                        <label class="form-check-label" for="flexCheckDefault">{"Double Name"}</label>
                </div>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                    <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"/>
                        <label class="form-check-label" for="flexCheckDefault">{"Active Status"}</label>
                </div>
                <select class="form-select mb-4" style=" margin: auto; width: 400px;" aria-label="Default select example"
                    onchange=self.link.callback(|e| {
                        if let ChangeData::Select(select) = e {
                            let value = select.value();
                            Msg::InputSelect(value)
                        } else {
                            Msg::InputSelect("No value".to_string())
                        }
                    })
                >
                    <option>{ "Last Active"}</option>
                        <option value="superhero">
                            { "Jira" }
                        </option>
                    <option value="normal">{ "Telkom Automation" }</option>
                </select>
                
                <div class="btn" style="justify-content: space-around; display: flex;"> 
                
                <button
                    type="button"
                    class="btn btn-primary mr-3" 
                    onclick=self.link.callback(|_| {
                        Msg::Login
                    })
                >
                    { "Start" }
                </button>

                <button
                    type="button"
                    class="btn btn-danger"
                    onclick=self.link.callback(|_| {
                        Msg::Login
                    })
                >
                    { "Stop" }
                </button>
                </div>
                
                </div>
                <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"Select Project"}</h5>
                                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body">
                            <img src="https://mdbcdn.b-cdn.net/img/new/avatars/2.webp" class="rounded-circle" style="width: 100px; height:100px"
                            alt="Avatar" />
                            <h6 style="padding: 10px;">{"Farrel Dava"}</h6>
                                
                                <div>
                                <select class="form-select mb-4" style=" margin: auto; width: 400px;" aria-label="Default select example"
                                    onchange=self.link.callback(|e| {
                                        if let ChangeData::Select(select) = e {
                                            let value = select.value();
                                            Msg::InputSelect(value)
                                        } else {
                                            Msg::InputSelect("No value".to_string())
                                        }
                                    })
                                >
                                    <option>{ "List Project"}</option>
                                    <option value="normal">{ "< Project 1 >" }</option>
                                    <option value="normal">{ "< Project 2 >" }</option>
                                    <option value="normal">{ "< Project 3 >" }</option>
                                </select>
                                </div>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                                <button type="button" class="btn btn-primary">{"Save changes"}</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
