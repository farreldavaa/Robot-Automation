
use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};

use yew_router::prelude::*;
use crate::router::route::AppRoute;
use crate::types::var::{
    UserAccount,
};


pub enum Msg {
}

pub struct Sidebar {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
    message: String,
}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        
        Self {
            link,
            fetch_task: None,
            error: None,
            message: String::from("Initial Message"),

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { "michael" }
            </div>
        }
    }
}
