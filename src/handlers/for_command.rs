pub fn handle(project_name: Option<&str>) -> String {

    if let Some(name) = project_name {
        return format!("{}", name)
    }

    return String::new()
}