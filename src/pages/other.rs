// use anyhow::Error;
// use yew::{prelude::*, callback};

// use yew::{
//     format::{ Json, Nothing },
//     prelude::*,
//     services::{
//         fetch::{FetchService, FetchTask, Request, Response},
//         ConsoleService,
//     },
// };
// use yew_router::prelude::*;
// use yew_router::agent::RouteRequest::ChangeRoute;

// use yew_router::prelude::*;
// use crate::router::route::AppRoute;
// use crate::types::var::{
//     UserAccount,
// };

// use crate::types::var::{
//     Users
// };

// use yew_router::prelude::*;
// // use serde::{
// //     Deserialize,
// //     Serialize,
// // };
// use crate::types::var::{
//     User,
//     Villain,
// };

// #[derive(Debug, Clone)]
// // pub struct Create{
// //     // `ComponentLink` is like a reference to a component.
// //     // It can be used to send messages to the component

// //     // DATA
// //     name: String,
// //     desc: String,
// //     api: String,
// //     email: String,
// //     token: String,
// //     platform: Option<String>,
// //     scheduler: i64,
// //     last_active: i64,
// //     check_active_status: bool,
// //     check_double_name: bool,
// //     check_double_email: bool,
// //     username: String,
// //     status: String,

// //     // SERVICES
// //     link: ComponentLink<Self>,
// //     fetch_task: Option<FetchTask>,
// //     router_agent: Box<dyn Bridge<RouteAgent>>,
// // }

// pub enum Msg {
//     RequestData,
//     GetData(Users),
//     GetDataString(String),
//     ResponseError(String),
//     RequestMoreData,
// }

// pub struct OtherPage {
//     fetch_task: Option<FetchTask>,
//     link: ComponentLink<Self>,
//     user: Users,
// }

// impl Component for OtherPage {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

//         Self {
//             fetch_task: None,
//             link,
//             // user: Users{

//             // name: String::from(""),
//             // desc: String::from(""),
//             // api: String::from(""),
//             // email: String::from(""),
//             // token: String::from(""),
//             // platform: String::from(""),
//             // scheduler: 0,
//             // last_active: 0,
//             // check_active_status: true,
//             // check_double_name: true,
//             // check_double_email: true,
//             // },
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
           
//            Msg::RequestData => {

//             // let user = Users {
//             //     name: Some(self.name.clone()),
//             //     desc: self.desc.clone(),
//             //     api: self.api.clone(),
//             //     email: self.email.clone(),
//             //     token: self.token.clone(),
//             //     platform: None,
//             //     scheduler: 0,
//             //     last_active: 0,
//             //     check_active_status: true,
//             //     check_double_name: true,
//             //     check_double_email: true,
//             // };
//                 // FETCHING....

//                 let request = Request::get("http://localhost:3000/attacks")
//                     // .header("access_token", get_access_token().unwrap_or_default())
//                     .body(Nothing)
//                     .expect("Could not build request.");
//                 let callback = 
//                     self.link.callback(|response: Response<Json<Result<Users, anyhow::Error>>>| {
//                         let (meta, Json(data)) = response.into_parts();
//                         // let status_number = meta.status.as_u16();

//                         match data {
//                             Ok(dataok) => {
//                                 ConsoleService::info(&format!("data response {:?}", &dataok));
//                                 Msg::GetData(dataok)
//                             }
//                             Err(error) => {
//                                 Msg::ResponseError(error.to_string())
//                             }
//                         }
//                     });
//                 let task = FetchService::fetch(request, callback).expect("failed to start request");

//                 self.fetch_task = Some(task);


//                 true
//             }
//             Msg::GetData(data) => {
//                 ConsoleService::info(&format!("data is {:?}", data));
//                 self.user = data;
//                 true
//             }
//             Msg::GetDataString(data_string) => {
//                 ConsoleService::info(&format!("data string is {:?}", data_string));
//                 true
//             }
//             Msg::ResponseError(text) => {
//                 ConsoleService::info(&format!("error is {:?}", text));
//                 true
//             }
//             Msg::RequestMoreData => {

//                 // FETCHING....

//                 let villain = Villain {
//                     name: String::from("arthur"),
//                     supervillain: String::from("joker"),
//                 };


//                 let request = Request::post("http://localhost:3000/attack")
//                     // .header("access_token", get_access_token().unwrap_or_default())
//                     .header("Content-Type", "application/json")
//                     .body(Json(&villain))
//                     .expect("Could not build request.");
//                 let callback = 
//                     self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
//                         let (meta, Json(data)) = response.into_parts();

//                         let status_number = meta.status.as_u16();

//                         ConsoleService::info(&format!("status is {:?}", status_number));

//                         match data {
//                             Ok(dataok) => {
//                                 ConsoleService::info(&format!("data response {:?}", &dataok));
//                                 Msg::GetDataString(dataok)
//                             }
//                             Err(error) => {
//                                 Msg::ResponseError(error.to_string())
//                             }
//                         }
//                     });
//                 let task = FetchService::fetch(request, callback).expect("failed to start request");

//                 self.fetch_task = Some(task);


//                 true
//             }
//         }
//     }

//     fn rendered(&mut self, first_render: bool) {
//         if first_render {
//             // self.link.send_message(Msg::RequestData);
//         }
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         // Should only return "true" if new properties are different to
//         // previously received properties.
//         // This component has no properties so we will always return "false".
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             // <div
//             //     style="
//             //         text-align: center;
//             //         height: 500px;
//             //         background: rgb(150,150,150);
//             //     "
//             // >
//             //     <div>
//             //         <button
//             //             class="display-2"
//             //             onclick=self.link.callback(|_| Msg::RequestData)
//             //         >
//             //             { "Get Data" }
//             //         </button>
//             //     </div>

//             //     <div>
//             //         {"user adalah "}
//             //         { self.user.name.clone().unwrap_or_default() }
//             //     </div>


//             //     <div>
//             //         <button
//             //             class="display-2"
//             //             onclick=self.link.callback(|_| Msg::RequestMoreData)
//             //         >
//             //             { "Get More Data" }
//             //         </button>
//             //     </div>

//             //     <div class="base">
//             //     <div class="homepage">
//             //         <div class="container-md mb-3" style="justify-content: space-between; display: flex; border-radius: 10px;">
//             //             <span>{"<Map 1>"}</span>
//             //         </div>
//             //         </div>
//             //     </div>
//             // </div>
//         }
//     }
// }
