use rustbreak::{FileDatabase, RustbreakError};
use rustbreak::deser::{Yaml};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Local, Datelike};
use serde::{Serialize, Deserialize};
use snafu::{ResultExt, IntoError, OptionExt};

use crate::domain::entities::{Billable, Project, BillableUnit};
use crate::domain::objects::{Quantity, Money, Task};
use crate::domain::errors::rustbreak_client::RustbreakClientError;
use crate::domain::errors::rustbreak_client::{ReadFailed, WriteFailed, SaveToFile, MalformedDateString, NoneError, UnexpectedTask};

type DB = FileDatabase<ProjectData, Yaml>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct BillableEntry {
    project_id: String,
    task: String,
    quantity: Quantity,
    date: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectData {
    billable: Vec<BillableEntry>,
    projects: HashMap<String, Project>
}

pub struct ProjectDataClient {
    db: DB
}

impl ProjectDataClient {
    pub fn new(path: String) -> ProjectDataClient {
        let db: DB = FileDatabase::create_at_path(path, ProjectData {
            billable: vec![],
            projects: HashMap::new(),
        }).unwrap();
        let _ = db.load();

        return ProjectDataClient {
            db
        }
    }

    pub fn add_project(&self, project_name: &String, unit_price: Money, unit: BillableUnit) -> Result<(), RustbreakClientError> {
        let project_id = self.get_project_id(&project_name);
        let project = Project{
            unit_price,
            unit,
            name: project_name.to_string(),
            tasks: HashSet::new()
        };
        let _ = self.db.write(|db| {
            db.projects.insert(project_id, project)
        }).map_err(|e| WriteFailed {  }.into_error(e))?;

        return Ok(());
    }

    pub fn add_task(&self, project_name: &String, task_name: &Task) -> Result<(), RustbreakClientError> {
        let project_id = self.get_project_id(&project_name);
        let _ = self.db.write(|db| {
            if let Some(p) = db.projects.get_mut(&project_id) {
                p.tasks.insert(task_name.to_string());
            }
            return false
        }).map_err(|e| WriteFailed { }.into_error(e))?;

        return Ok(());
    }


    pub fn add_billable_entry(&self, project_name: &String, task: &String, quantity: Quantity, date: Option<DateTime<Local>>) -> Result<(), RustbreakClientError> {
        let project_id = self.get_project_id(&project_name);
        let is_known_task = self.task_exists(&project_id, &task).map_err(|e| ReadFailed { }.into_error(e))?;

        if !is_known_task {
            return Err(UnexpectedTask { task: task.to_string() }.build());
        }

        let date_str = match date {
            None => Local::now().to_string(),
            Some(d) => d.to_string()
        };

        let billable = BillableEntry {
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

    pub fn get_monthly_billing(&self, project_name: &String, date: Option<DateTime<Local>>) -> Result<Vec<Billable>, RustbreakClientError> {
        let project_id = self.get_project_id(&project_name);
        let month = match date {
            None => Local::now().month(),
            Some(d) => d.month()
        };

        return self.db.read( |db| {
           Ok(db.billable.clone()
               .into_iter()
               .filter_map(|e| Some(Billable{ date: e.date.parse::<DateTime<Local>>().map_err(move |_| MalformedDateString).ok()?, project_id: e.project_id, quantity: e.quantity, task: e.task }))
               .filter(|e| e.project_id == project_id)
               .filter(|e| e.date.month() == month)
               .collect())
        }).map_err(|e| ReadFailed { }.into_error(e))?;
    }

    pub fn write_to_file(&self) -> Result<(), RustbreakClientError> {
        self.db.save().map_err(|e| SaveToFile { }.into_error(e))
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
    use crate::domain::entities::BillableUnit::Day;

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
        let client = ProjectDataClient::new(DB_FILE.to_string());
        client.add_project(project_name, 80, Day);

        let _ = client.db.read(|db| {
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

        let client = ProjectDataClient::new(DB_FILE.to_string());
        client.add_project(project_name, 80, Day);
        client.add_task(project_name, task_name);

        let _ = client.db.read(|db| {
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

        let client = ProjectDataClient::new(DB_FILE.to_string());
        client.add_project(project_name, 80, Day);
        client.add_task(project_name, expected_task);

        let _ = client.add_billable_entry(project_name, expected_task, 8.0, Some(expected_date));
        let _ = client.add_billable_entry(project_name, unexpected_task, 8.0, Some(expected_date));

        let _ = client.db.read(|db| {
            assert_eq!(db.billable[0], expected);
            assert_eq!(db.billable.len(), 1);
        });
    }
    #[test]
    fn test_add_billable_entry_default_to_current_date() {
        let project_name = &MOCK_PROJECT_NAME.to_string();
        let task_name = &EXPECTED_TASK_NAME.to_string();

        let expected_date_str_substring = Local::now().to_string()[..10].to_string();

        let client = ProjectDataClient::new(DB_FILE.to_string());
        client.add_project(project_name, 80, Day);
        client.add_task(project_name, task_name);

        let _ = client.add_billable_entry(project_name, task_name, 8.0, None);

        let _ = client.db.read(|db| {
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

        let client = ProjectDataClient::new(DB_FILE.to_string());


        client.add_project(project_name, 80, Day);
        client.add_task(project_name, task_name);

        let _ = client.add_billable_entry(project_name, task_name, 8.0, Some(expected_date1));
        let _ = client.add_billable_entry(project_name, task_name, 7.0, Some(expected_date2));
        let _ = client.add_billable_entry(project_name, task_name, 1.0, Some(expected_date3));

        let res = client.get_monthly_billing(project_name,  None).unwrap();

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

        let target_date = "2020-09-10 22:09:24.269707 +02:00".parse::<DateTime<Local>>().unwrap();

        let client = ProjectDataClient::new(DB_FILE.to_string());

        client.add_project(project_name, 80, Day);
        client.add_task(project_name, task_name);

        let _ = client.add_billable_entry(project_name, task_name, 8.0, Some(expected_date1));
        let _ = client.add_billable_entry(project_name, task_name, 7.0, Some(expected_date2));
        let _ = client.add_billable_entry(project_name, task_name, 1.0, Some(expected_date3));

        let res = client.get_monthly_billing(project_name,  Some(target_date)).unwrap();


        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Billable { project: MOCK_PROJECT_ID.to_string(), task: task_name.to_string(), quantity: 1.0, date: expected_date3 });
    }
}