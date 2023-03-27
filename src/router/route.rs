use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/projects"]
    Project,
    #[to="/createbot"]
    CreateBot,
    #[to="/login"]
    Login,
    #[to="/schedules"]
    Schedules,
    #[to="/setting/{idProject}"]
    InputPage {idProject : String},
    // #[to="/other"]
    // Other,
    #[to="/"]
    Home,
}