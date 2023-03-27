use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub name: Option<String>,
    pub superhero: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Villain {
    pub name: String,
    pub supervillain: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Users {
    pub name: String,
    pub description: String,
    pub cloudSessionToken: String,
    pub platformApiKey: String,
    pub platformEmail: String,
    pub platformType: String,
    pub schedule: i64,
    pub lastActive: i64,
    pub active: bool,
    pub checkActiveStatus: bool,
    pub checkDoubleEmail: bool,
    pub checkDoubleName: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectId{
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsersData {
    #[serde(rename = "_id")]
    pub id: ProjectId,
    pub name: String,
    pub description: String,
    pub platformEmail: String,
    pub platformApiKey: String,
    pub platformType: String,
    pub cloudSessionToken: String,
    pub active: bool,
    pub schedule: i64,
    pub lastActive: i64,
    pub checkDoubleName: bool,
    pub checkDoubleEmail: bool,
    pub checkActiveStatus: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectList {
    pub list: Option<Vec<UsersData>>,
    pub world: Option<String>,
    pub error_description: Option<String>,
    // other_data: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Schedule {
    pub task: String,
    pub superhero: String,
    pub is_on_going: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SchedulesData {
    pub list: Option<Vec<Schedule>>,
    pub world: Option<String>,
    pub error_description: Option<String>,
    // other_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortReturnValueInner{
    #[serde(rename = "$oid")]
    id: String, }

#[derive(Debug, Serialize, Deserialize)]
pub struct PostReturnValue {
    insertedId: PortReturnValueInner
}

#[derive(Serialize, Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub status: String,
}

#[derive(Clone)]
pub struct Superhero {
    pub name: String,
    pub age: u8,
}
impl Default for Superhero {
    fn default() -> Superhero {
        Superhero{
            name: String::from("batman"),
            age: 35
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Localstorage {
    pub user: String,
}

pub fn get_message() -> String {
    String::from("message from function")
}
