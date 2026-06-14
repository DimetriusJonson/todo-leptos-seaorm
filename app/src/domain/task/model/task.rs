use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Validate)]
pub struct Task {
    pub id: Option<i32>,
    #[validate(required, length(min = 3))]
    pub title: Option<String>,
    pub description: Option<String>,
    #[validate(required, length(min = 1, code = "required"))]
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

impl Task {
    #[cfg(feature = "ssr")]
    pub fn fix_completed_at(&mut self) -> &mut Self {
        use time::OffsetDateTime;

        if let Some(completed_at) = &self.completed_at
            && (completed_at == "on" || completed_at == "true" || completed_at == "checked")
        {
            let now_utc = OffsetDateTime::now_utc();
            let rfc2822_string = now_utc.format(&time::format_description::well_known::Rfc2822).expect("failed format now utc");
            self.completed_at = Some(rfc2822_string);
        }

        self
    }

    pub fn priority_name(&self) -> String {
        match &self.priority {
            Some(priority) => Self::priority_by_name(priority),
            None => "".to_owned(),
        }
    }

    pub fn priority_by_name(value: &str) -> String {
        let res = match value {
            "C" => "Критический",
            "H" => "Высокий",
            "N" => "Нормальный",
            "L" => "Низкий",
            _ => "",
        };

        res.to_owned()
    }
}

pub fn filter_task(task: &Task, filter: &Option<String>) -> bool {
    if let Some(filter) = filter {
        return match filter.as_str() {
            "Completed" => task.completed_at.is_some(),
            "Uncompleted" => task.completed_at.is_none(),
            _ => true,
        };
    }

    true
}

pub fn sort_task(task1: &Task, task2: &Task, sort_kind: &Option<String>) -> Ordering {
    if let Some(sort_kind) = sort_kind {
        return match sort_kind.as_str() {
            "Title" => task1.title.cmp(&task2.title),
            "Priority" => task1.priority_name().cmp(&task2.priority_name()),
            _ => task1.id.cmp(&task2.id),
        };
    }

    task1.id.cmp(&task2.id)
}
