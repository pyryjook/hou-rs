use rustbreak::{FileDatabase, Database, RustbreakError};
use rustbreak::deser::Yaml;
use rustbreak::backend::FileBackend;
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

type DB = FileDatabase<ProjectData, Yaml>;
type Money = u32;


#[derive(Debug, Serialize, Deserialize, Clone)]
enum BillableUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BillableEntry {
    project_id: String,
    amount: Money,
    date: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Project {
    name: String,
    unit_price: Money,
    unit: BillableUnit,
    tasks: HashSet<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectData {
    billable: Vec<BillableEntry>,
    projects: HashMap<String, Project>
}

struct ProjectDataService {
    db: DB
}

impl ProjectDataService {
    pub fn new(path: String) -> ProjectDataService {
        let db: DB = FileDatabase::create_at_path(path, ProjectData {
            billable: vec![],
            projects: HashMap::new(),
        }).unwrap();
        let _ = db.load();

        return ProjectDataService {
            db
        }
    }

    pub fn add_project(&self, project_name: String, unit_price: Money, unit: BillableUnit) {
        let project_id = self.get_project_id(&project_name);
        let project = Project{
            unit_price,
            unit,
          name: project_name,
            tasks: HashSet::new()
        };
        let _ = self.db.write(|db| {
            db.projects.insert(project_id, project)
        });

        self.db.save();
        return ()
    }

    pub fn add_task(&self, project_name: String, task_name: String) {
        let project_id = self.get_project_id(&project_name);
        let _ = self.db.write(|db| {
            if let Some(p) = db.projects.get_mut(&project_id) {
                return p.tasks.insert(task_name)
            }
            return false
        });

        self.db.save();
    }

    fn get_project_id(&self, project_name: &String) -> String {
        return project_name.to_ascii_lowercase();
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::services::file_database_service::BillableUnit::Day;

    #[test]
    fn test_add_project() {
        let service = ProjectDataService::new("test_helpers/db.yaml".to_string());
        service.add_project("Foo".to_string(), 80, Day)
    }

    #[test]
    fn test_add_task() {
        let service = ProjectDataService::new("test_helpers/db.yaml".to_string());
        service.add_task("Foo".to_string(), "development".to_string())
    }
}