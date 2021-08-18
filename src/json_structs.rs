// json_struct.rs contains all the structs json_serde uses to

pub mod structs {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Users {
        pub data: Vec<UsersData>,
        #[serde(rename = "totalCount")]
        pub total_count: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UsersData {
        pub email: String,
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        pub id: u64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Sheets {
        pub data: Vec<SheetsData>,
        #[serde(rename = "totalCount")]
        pub total_count: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SheetsData {
        pub name: String,
        pub id: u64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Events {
        pub data: Vec<EventsData>,
        #[serde(rename = "nextStreamPosition")]
        pub next_position: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EventsData {
        pub action: String,
        #[serde(rename = "eventTimestamp")]
        pub time: String,
        #[serde(rename = "objectId")]
        pub object_id: u64,
        #[serde(rename = "objectType")]
        pub object_type: String,
        pub source: String,
        #[serde(rename = "userId")]
        pub user_id: u64,
        #[serde(rename = "additionalDetails")]
        pub details: Option<serde_json::Value>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Reports {
        pub data: Vec<ReportsData>,
        #[serde(rename = "totalCount")]
        pub total_count: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ReportsData {
        pub id: u64,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Workspaces {
        pub data: Vec<WorkspacesData>,
        #[serde(rename = "totalCount")]
        pub total_count: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WorkspacesData {
        pub id: u64,
        pub name: String,
    }
}
