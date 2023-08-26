use std::{fs, io::Write, path::Path};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An entry for the statsfile.
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct StatsEntry {
    /// What version of stats file this is, useful if the format changes in the future.
    pub entry_version: u32,
    /// The timstamp of the healthcheck.
    pub timestamp: DateTime<Utc>,
    /// The url that was pinged during this healthcheck.
    pub pinged_url: String,
    /// The 'request_state' used for the healthcheck.
    pub request_state: String,
    /// The result of the healthcheck itself.
    pub ping_result: PingResult,
    /// If an error occurred during the ping, contains the error, otherwise None.
    pub ping_error: Option<String>,
    /// The result of sending the email.
    pub email_result: EmailResult,
    /// If an error occurred whilst sending the email, contains the error, otherwise Email.
    pub email_error: Option<String>,
}

/// The result of a healthcheck ping.
#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum PingResult {
    /// A sucessful ping result.
    Success,
    /// A failed ping result.
    Failure,
}

/// The result of emailing the healthcheck result.
#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum EmailResult {
    /// The email was sent without issues.
    SentSuccessfully,
    /// A try was made to send an email but it failed with the wrapped error.
    FailedToSend,
    /// No email was sent.
    NotSent,
}

/// Log an entry to the stats file at the provided `stats_file_path`.
/// Will create the file if it does not already exist.
pub fn log_entry(
    stats_file_path: &str,
    state: String,
    ping_url: String,
    ping_result: PingResult,
    ping_error: Option<String>,
    email_result: EmailResult,
    email_error: Option<String>,
) {
    let entry = StatsEntry {
        entry_version: 1,
        timestamp: Utc::now(),
        pinged_url: ping_url,
        request_state: state,
        ping_result,
        ping_error,
        email_result,
        email_error,
    };

    let path = Path::new(stats_file_path);
    if path.exists() {
        if !path.is_file() {
            panic!("Provided stats file path {path:?} is not a file!");
        }

        // The path exists and is a file, let's append to it.

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(path)
            .expect("Failed to open stats file");

        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(vec![]);
        writer.serialize(entry).expect("Failed to serialize entry");
        let row = writer
            .into_inner()
            .expect("Failed to retrieve underlying csv writer");

        file.write_all(row.as_slice())
            .expect("Failed to append stats entry to file");
    } else {
        println!("Stats file does not exist, creating at {path:?}");
        let mut writer = csv::WriterBuilder::new()
            .from_path(path)
            .expect("Failed to open stats output file");

        writer.serialize(entry).expect("Failed to serialize entry");

        writer.flush().expect("Failed to flush writer");
    };
}

/// Reads the contents of the stats file or panics if unable to do so.
pub fn load_from_file(stats_file_path: &str) -> Vec<StatsEntry> {
    let path = Path::new(stats_file_path);
    if !path.exists() {
        panic!("Stats file does not exist at path {path:?}");
    }

    if !path.is_file() {
        panic!("Provided stats file path {path:?} is not a file!");
    }

    // The path exists and is a file, let's append to it.

    let file_contents = fs::read_to_string(path).expect("Failed to open stats file");

    csv::Reader::from_reader(file_contents.as_bytes())
        .deserialize()
        .collect::<Result<Vec<StatsEntry>, csv::Error>>()
        .expect("Failed to parse file contents")
}
