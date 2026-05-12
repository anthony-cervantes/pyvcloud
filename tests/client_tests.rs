use pyvcloud::{ApiVersion, Client, Query, SortDirection, Task, TaskStatus};

#[test]
fn validates_supported_versions() {
    assert_eq!(ApiVersion::new("36.0").unwrap().as_str(), "36.0");
    assert!(ApiVersion::new("1.0").is_err());
}

#[test]
fn builds_query_paths() {
    let path = Query::new("vm")
        .filter("name==web")
        .field("name")
        .sort("name", SortDirection::Desc)
        .page(2)
        .page_size(50)
        .path();
    assert_eq!(
        path,
        "/api/query?type=vm&filter=name==web&fields=name&sortdesc=name&page=2&pageSize=50"
    );
}

#[test]
fn parses_task_xml() {
    let task = Task::from_xml(r#"<Task href="https://vcd.example/api/task/1" id="urn:vcloud:task:1" name="powerOn" operation="Power on" status="success"/>"#).unwrap();
    assert_eq!(task.status, TaskStatus::Success);
    assert_eq!(task.name.as_deref(), Some("powerOn"));
}

#[test]
fn builds_default_client() {
    let client = Client::new("https://vcd.example/").unwrap();
    assert_eq!(client.base_url().as_str(), "https://vcd.example/");
    assert_eq!(client.api_version().as_str(), "36.0");
}
