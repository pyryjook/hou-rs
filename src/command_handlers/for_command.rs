//use crate::builders::config_builder::ConfigBuilder;


pub fn handle(project_name: Option<&str>) -> String {
//    let config = ConfigBuilder::new();

    if let Some(name) = project_name {
        return format!("{}", name)
    }

    return String::new()
}