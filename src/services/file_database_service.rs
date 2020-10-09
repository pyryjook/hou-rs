use rustbreak::{FileDatabase, Database, RustbreakError};
use rustbreak::deser::{Yaml, Bincode};
use rustbreak::backend::FileBackend;
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Local, Date, Datelike};
use serde::{Serialize, Deserialize};

type DB = FileDatabase<ProjectData, Yaml>;
type Money = u16;
type Quantity = f32;


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
    quantity: Quantity,
    date: String
}

#[derive(Debug)]
struct Billable {
    project_id: String,
    quantity: Quantity,
    date: DateTime<Local>
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


    pub fn add_billable_entry(&self, project_name: String, task_name: String, quantity: Quantity, date: Option<Date<Local>>) -> Result<(), RustbreakError> {
        let project_id = self.get_project_id(&project_name);
        let is_known_task = self.task_exists(&project_id, &task_name)?;
        if !is_known_task {
            return Err(RustbreakError::Poison)
        }
        let date_str = match date {
            None => Local::now().to_string(),
            Some(d) => d.to_string()
        };

        let billable = BillableEntry{
            project_id,
            quantity,
            date: date_str
        };

        let _ = self.db.write(|db| {
            db.billable.insert(db.billable.len(), billable)
        });

        self.db.save();

        return Ok(());
    }

    pub fn get_monthly_billing(&self, project_name: String, date: Option<Date<Local>>) -> Result<Vec<Billable>, RustbreakError> {
        let project_id = self.get_project_id(&project_name);
        let month = match date {
            None => Local::now().month(),
            Some(d) => d.month()
        };

        return self.db.read( |db| {
           db.billable.clone()
               .into_iter()
               .filter_map(|e| Some(Billable{ date: e.date.parse::<DateTime<Local>>().ok()?, project_id: e.project_id, quantity: e.quantity }))
               .filter(|e| e.project_id == project_id)
               .filter(|e| e.date.month() == month)
               .collect()
        });
    }

    fn task_exists(&self, project_id: &String, task_name: &String) -> Result<bool, RustbreakError> {
        return self.db.read(|db| {
            if let Some(p) = db.projects.get(project_id) {
                return p.tasks.contains(task_name);
            }

            return false
        });
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

    #[test]
    fn test_add_billable_entry() {
        let service = ProjectDataService::new("test_helpers/db.yaml".to_string());
        let _ = service.add_billable_entry("Foo".to_string(), "development".to_string(), 8.0, None);
    }

    #[test]
    fn test_get_monthly_billing() {
        let service = ProjectDataService::new("test_helpers/db.yaml".to_string());
        let res = service.get_monthly_billing("Foo".to_string(),  None).unwrap();

        println!("{:?}", res)
    }
}