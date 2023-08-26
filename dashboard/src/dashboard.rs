use chrono::{Datelike, Timelike};
use healthcheck_common::stats_file::{self, EmailResult, PingResult, StatsEntry};
use rocket::State;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

use crate::config::Config;

const DASHBOARD_TEMPLATE_NAME: &str = "dashboard";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableEntry {
    version: u32,
    timestamp: String,
    request_state: String,
    full_url: String,
    ping_result: PingResult,
    ping_error: String,
    ping_color: DisplayColor,
    email_result: EmailResult,
    email_error: String,
    email_color: DisplayColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum DisplayColor {
    Ok,
    Error,
}

impl From<PingResult> for DisplayColor {
    fn from(value: PingResult) -> Self {
        match value {
            PingResult::Success => Self::Ok,
            PingResult::Failure => Self::Error,
        }
    }
}

impl From<EmailResult> for DisplayColor {
    fn from(value: EmailResult) -> Self {
        match value {
            EmailResult::FailedToSend => Self::Error,
            _ => Self::Ok,
        }
    }
}

impl From<StatsEntry> for TableEntry {
    fn from(value: StatsEntry) -> Self {
        Self {
            version: value.entry_version,
            timestamp: format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{02}",
                value.timestamp.year(),
                value.timestamp.month(),
                value.timestamp.day(),
                value.timestamp.hour(),
                value.timestamp.second()
            ),
            request_state: value.request_state,
            full_url: value.pinged_url,
            ping_result: value.ping_result.clone(),
            ping_error: value.ping_error.unwrap_or(String::from("No error")),
            ping_color: value.ping_result.into(),
            email_result: value.email_result.clone(),
            email_error: value.email_error.unwrap_or(String::from("No error")),
            email_color: value.email_result.into(),
        }
    }
}

#[get("/")]
pub fn get_dashboard(config: &State<Config>) -> Template {
    let mut file_entries = stats_file::load_from_file(&config.stats_file);
    let show_table = file_entries.is_empty().clone() == false;

    file_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let table_entries: Vec<TableEntry> = file_entries.into_iter().map(|e| e.into()).collect();

    Template::render(
        DASHBOARD_TEMPLATE_NAME,
        context! {
            entries: table_entries,
            show_table: show_table
        },
    )
}
