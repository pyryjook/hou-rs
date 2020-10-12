use rustbreak::{FileDatabase, RustbreakError};
use rustbreak::deser::{Yaml};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Local, Datelike};
use serde::{Serialize, Deserialize};

type DB = FileDatabase<ProjectData, Yaml>;
type Money = u16;
type Quantity = f32;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum BillableUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct BillableEntry {
    project_id: String,
    task: String,
    quantity: Quantity,
    date: String
}

#[derive(Debug, PartialEq)]
struct Billable {
    project_id: String,
    task: String,
    quantity: Quantity,
    date: DateTime<Local>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    pub fn add_project(&self, project_name: &String, unit_price: Money, unit: BillableUnit) {
        let project_id = self.get_project_id(&project_name);
        let project = Project{
            unit_price,
            unit,
            name: project_name.to_string(),
            tasks: HashSet::new()
        };
        let _ = self.db.write(|db| {
            db.projects.insert(project_id, project)
        });
    }

    pub fn add_task(&self, project_name: &String, task_name: &String) {
        let project_id = self.get_project_id(&project_name);
        let _ = self.db.write(|db| {
            if let Some(p) = db.projects.get_mut(&project_id) {
                return p.tasks.insert(task_name.to_string())
            }
            return false
        });
    }


    pub fn add_billable_entry(&self, project_name: &String, task: &String, quantity: Quantity, date: Option<DateTime<Local>>) -> Result<(), RustbreakError> {
        let project_id = self.get_project_id(&project_name);
        let is_known_task = self.task_exists(&project_id, &task)?;

        if !is_known_task {
            return Err(RustbreakError::Poison)
        }

        let date_str = match date {
            None => Local::now().to_string(),
            Some(d) => d.to_string()
        };

        let billable = BillableEntry{
            project_id,
            task: task.to_string(),
            quantity,
            date: date_str
        };

        let _ = self.db.write(|db| {
            db.billable.insert(db.billable.len(), billable)
        });

        return Ok(());
    }

    pub fn get_monthly_billing(&self, project_name: &String, date: Option<DateTime<Local>>) -> Result<Vec<Billable>, RustbreakError> {
        let project_id = self.get_project_id(&project_name);
        let month = match date {
            None => Local::now().month(),
            Some(d) => d.month()
        };

        return self.db.read( |db| {
           db.billable.clone()
               .into_iter()
               .filter_map(|e| Some(Billable{ date: e.date.parse::<DateTime<Local>>().ok()?, project_id: e.project_id, quantity: e.quantity, task: e.task }))
               .filter(|e| e.project_id == project_id)
               .filter(|e| e.date.month() == month)
               .collect()
        });
    }

    pub fn write_to_file(&self) -> Result<(), RustbreakError> {
        self.db.save()
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

    const DB_FILE: &str = "test_helpers/db.yaml";
    const MOCK_PROJECT_NAME: &str = "Foo";
    const MOCK_PROJECT_ID: &str = "foo";
    const EXPECTED_TASK_NAME: &str = "development";
    const UNEXPECTED_TASK_NAME: &str = "destruction";

    #[test]
    fn test_add_project() {
        let project_name = &MOCK_PROJECT_NAME.to_string();

        let expected = Project {
            name: project_name.to_string(),
            unit_price: 80,
            unit: Day,
            tasks: HashSet::new()
        };
        let service = ProjectDataService::new(DB_FILE.to_string());
        service.add_project(project_name, 80, Day);

        let _ = service.db.read(|db| {
            assert_eq!(db.projects.get(MOCK_PROJECT_ID), Some(&expected))
        });

    }

    #[test]
    fn test_add_task() {
        let project_name = &MOCK_PROJECT_NAME.to_string();
        let task_name = &EXPECTED_TASK_NAME.to_string();

        let mut set = HashSet::new();
        set.insert(task_name.to_string());

        let expected = Project {
            name: project_name.to_string(),
            unit_price: 80,
            unit: Day,
            tasks: set
        };
        let service = ProjectDataService::new(DB_FILE.to_string());
        service.add_project(project_name, 80, Day);
        service.add_task(project_name, task_name);

        let _ = service.db.read(|db| {
            assert_eq!(db.projects.get(MOCK_PROJECT_ID), Some(&expected))
        });
    }

    #[test]
    fn test_add_billable_entry() {
        let project_name = &MOCK_PROJECT_NAME.to_string();

        let expected_date = Local::now();
        let expected_task = &EXPECTED_TASK_NAME.to_string();
        let unexpected_task = &UNEXPECTED_TASK_NAME.to_string();
        let expected = BillableEntry {
            project_id: MOCK_PROJECT_ID.to_string(),
            quantity: 8.0,
            task: expected_task.to_string(),
            date: expected_date.to_string()
        };
        let service = ProjectDataService::new(DB_FILE.to_string());
        service.add_project(project_name, 80, Day);
        service.add_task(project_name, expected_task);

        let _ = service.add_billable_entry(project_name, expected_task, 8.0, Some(expected_date));
        let _ = service.add_billable_entry(project_name, unexpected_task, 8.0, Some(expected_date));

        let _ = service.db.read(|db| {
            assert_eq!(db.billable[0], expected);
            assert_eq!(db.billable.len(), 1);
        });
    }
    #[test]
    fn test_add_billable_entry_default_to_current_date() {
        let project_name = &MOCK_PROJECT_NAME.to_string();
        let task_name = &EXPECTED_TASK_NAME.to_string();

        let expected_date_str_substring = Local::now().to_string()[..10].to_string();
        let service = ProjectDataService::new(DB_FILE.to_string());
        service.add_project(project_name, 80, Day);
        service.add_task(project_name, task_name);

        let _ = service.add_billable_entry(project_name, task_name, 8.0, None);

        let _ = service.db.read(|db| {
            assert_eq!(db.billable[0].date.starts_with(&expected_date_str_substring), true)
        });
    }

    #[test]
    fn test_get_monthly_billing_current_month() {
        let project_name = &MOCK_PROJECT_NAME.to_string();
        let task_name = &EXPECTED_TASK_NAME.to_string();

        let expected_date1 = "2020-10-11 22:09:24.269707 +02:00".parse::<DateTime<Local>>().unwrap();
        let expected_date2 = "2020-10-12 22:09:24.269707 +02:00".parse::<DateTime<Local>>().unwrap();
        let expected_date3 = "2020-09-12 22:09:24.269707 +02:00".parse::<DateTime<Local>>().unwrap();
        let service = ProjectDataService::new(DB_FILE.to_string());


        service.add_project(project_name, 80, Day);
        service.add_task(project_name, task_name);

        let _ = service.add_billable_entry(project_name, task_name, 8.0, Some(expected_date1));
        let _ = service.add_billable_entry(project_name, task_name, 7.0, Some(expected_date2));
        let _ = service.add_billable_entry(project_name, task_name, 1.0, Some(expected_date3));

        let res = service.get_monthly_billing(project_name,  None).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Billable { project_id: MOCK_PROJECT_ID.to_string(), task: task_name.to_string(), quantity: 8.0, date: "2020-10-11T22:09:24.269707+02:00".parse::<DateTime<Local>>().unwrap() });
        assert_eq!(res[1], Billable { project_id: MOCK_PROJECT_ID.to_string(), task: task_name.to_string(), quantity: 7.0, date: "2020-10-12T22:09:24.269707+02:00".parse::<DateTime<Local>>().unwrap() });
    }

    #[test]
    fn test_get_monthly_billing_explicit_month() {
        let project_name = &MOCK_PROJECT_NAME.to_string();
        let task_name = &EXPECTED_TASK_NAME.to_string();

        let expected_date1 = "2020-10-11 22:09:24.269707 +02:00".parse::<DateTime<Local>>().unwrap();
        let expected_date2 = "2020-10-12 22:09:24.269707 +02:00".parse::<DateTime<Local>>().unwrap();
        let expected_date3 = "2020-09-12 22:09:24.269707 +02:00".parse::<DateTime<Local>>().unwrap();

        let taget_date =  "2020-09-10 22:09:24.269707 +02:00".parse::<DateTime<Local>>().unwrap();
        let service = ProjectDataService::new(DB_FILE.to_string());


        service.add_project(project_name, 80, Day);
        service.add_task(project_name, task_name);

        let _ = service.add_billable_entry(project_name, task_name, 8.0, Some(expected_date1));
        let _ = service.add_billable_entry(project_name, task_name, 7.0, Some(expected_date2));
        let _ = service.add_billable_entry(project_name, task_name, 1.0, Some(expected_date3));

        let res = service.get_monthly_billing(project_name,  Some(taget_date)).unwrap();


        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Billable { project_id: MOCK_PROJECT_ID.to_string(), task: task_name.to_string(), quantity: 1.0, date: expected_date3 });
    }
}