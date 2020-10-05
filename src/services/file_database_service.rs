use rustbreak::{FileDatabase, Database, RustbreakError};
use rustbreak::deser::Yaml;
use rustbreak::backend::FileBackend;
use std::collections::HashMap;

type DB = FileDatabase<ServerData, Yaml>;
type Money = u32;


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum BillableUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour
}

#[derive(Debug, Serialize, Deserialize, Clone, FromForm)]
struct Project {
    name: String,
    unit_price: Money,
    unit: BillableUnit
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServerData {
    hours: Vec<HourEntry>,
    projects: HashMap<String, Project>
}

struct ProjectDataService {
    db: DB
};

impl ProjectDataService {
    fn new(path: String) -> DatabaseService {
        let db: DB = FileDatabase::from_path(path, ServerData {
            hours: vec![],
            users: HashMap::new(),
        }).unwrap();
        let _ = db.load();
    }
}