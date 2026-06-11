pub struct TaskRoutes;

impl TaskRoutes {
    pub fn base_segment() -> &'static str {
        "task"
    }

    pub fn base_url() -> &'static str {
        "/task"
    }

    pub fn create_url() -> &'static str {
        "/task/create"
    }

    pub fn create_segment() -> &'static str {
        "create"
    }

    pub fn label() -> &'static str {
        "Task"
    }

    pub fn details_url(id: i32) -> String {
        format!("/task/{id}")
    }

    pub fn edit_url(id: i32) -> String {
        format!("/task/{id}/edit")
    }
}
