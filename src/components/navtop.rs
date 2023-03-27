use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;
use crate::router::route::AppRoute;
use std::rc::Rc;

use crate::store::store::{
    CounterStore,
    CounterOutput,
    CounterInput,
    State,
};

pub enum Msg {
    AddOne,
    GetLogin,
}


pub struct Navtop {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Navtop {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }

            Msg::GetLogin => {
                true
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

        type Anchor = RouterAnchor<AppRoute>;

        html! {
            <>
            <div class="header">       
                <input type="checkbox" class="openSidebarMenu" id="openSidebarMenu"/>
                    <label for="openSidebarMenu" class="sidebarIconToggle">
                        <div class="spinner diagonal part-1"></div>
                        <div class="spinner horizontal"></div>
                        <div class="spinner diagonal part-2"></div>
                    </label>
                <div class="sidebar" id="sidebarMenu">
                    <ul class="sidebarMenuInner">
                        <li>{"Digital Business"} <span>{"Web Development"}</span></li>
                        <li class="sidebarDrop"><a>{"Robot Automation"}</a>
                            <ul class="serviceProjects">
                                <li>
                                <Anchor route=AppRoute::Home>
                                            {"Homepage"}
                                    </Anchor>
                                </li>
                            </ul>
                            <ul class="serviceProjects">
                                <li>
                                <Anchor route=AppRoute::Project>
                                            {"Project"}
                                    </Anchor>
                                </li>
                            </ul>
                        </li>
                        <li><a href="https://instagram.com/plavookac" target="_blank">{"Connector"}</a></li>
                    </ul>
                </div>
            <h5 class="new-navbar">{"Robot Automation"}</h5>
            </div>
            // <div class="base-navtop">
            //     <div class="navbar">
            //             <div class="navbar-option" style="justify-content: space-around; display: flex;">
            //                     <div class="ml-3 mr-3 pr-3 pl-3" style="padding-right: 10px">
            //                         <Anchor route=AppRoute::Home>
            //                             <label class="link btn btn-light pl-5" style="">
            //                                 {"Homepage"}
            //                             </label>
            //                         </Anchor>
            //                     </div>
            //                     <div class="ml-3 mr-3 pr-3 pl-3" style="">
            //                         <Anchor route=AppRoute::Project>
            //                             <label class="link btn btn-light pl-5" style="">
            //                                 {"Project"}
            //                             </label>
            //                         </Anchor>
            //                     </div>
            //             </div>
            //             // </div>
            //     </div>
            //     // <div style="text-decoration: none; color: rgb(100,100,100);">
            //     //     <Anchor route=AppRoute::Schedules>
            //     //         <p class="link" style=" text-decoration: none!important; color: rgb(100,100,100);">
            //     //             {"Schedules"}
            //     //         </p>
            //     //     </Anchor>
            //     // </div>
            //     // <div style=" text-decoration: none; color: rgb(100,100,100);">
            //     //     <Anchor route=AppRoute::Login>
            //     //         <p class="link" style=" text-decoration: none!important; color: rgb(100,100,100);">
            //     //             {"Login"}
            //     //         </p>
            //     //     </Anchor>
            //     // </div>

            // </div>
            </>
        }
    }
}
