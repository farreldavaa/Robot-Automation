use yew::prelude::*;

// use crate::pages::content::Content;

use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};

use yew_router::prelude::*;
use crate::router::route::AppRoute;
use crate::types::var::{
    UserAccount,
};

pub enum Msg {
    AddOne,
    InputText(String),
}

pub struct PageInput {

    username: String,
    status: String,
}

pub struct HomePage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    message: String,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Homepage");
        Self {
            link,
            value: 0,
            message: String::from("Initial Message"),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
            //     input.focus();
            // }

            ConsoleService::info("First Render");
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
            Msg::InputText(data) => {
                self.message = data;
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
            <div class="base">
                <div class="homepage">
                    <div class="container-md mb-3" style="justify-content: space-between; display: flex; border-radius: 10px;">
                        <span>{"<Map 1>"}</span>
                            <Anchor route=AppRoute::InputPage>
                                <button type="button" class="btn btn-primary btn-sm pl-3">{"Setting"}
                                </button>
                        </Anchor>
                    </div>
                    <div class="container-md mb-3" style="justify-content: space-between; display: flex; border-radius: 10px;">
                        <span>{"<Map 2>"}</span>
                            <Anchor route=AppRoute::InputPage>
                                <button type="button" class="btn btn-primary btn-sm pl-3">{"Setting"}
                                </button>
                            </Anchor>
                    </div>
                    <div class="container-md mb-3" style="justify-content: space-between; display: flex; border-radius: 10px;">
                        <span>{"<Map 3>"}</span>
                            <Anchor route=AppRoute::InputPage>
                                <button type="button" class="btn btn-primary btn-sm pl-3">{"Setting"}
                                </button>
                            </Anchor>
                    </div>
                    </div>
                </div>
        }
    }
}
