use thiserror::Error;

#[derive(Error, Debug)]
pub enum CutError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Invalid timecode string: '{0}' (expected HH:MM:SS.mmm or seconds)")]
    InvalidTimecode(String),

    #[error("Invalid segment range: start {start}s must be less than end {end}s")]
    InvalidRange { start: f64, end: f64 },

    #[error("FFmpeg execution failed with code {code:?}: {stderr}")]
    FfmpegExecution {
        code: Option<i32>,
        stderr: String,
    },

    #[error("Media probe error: {0}")]
    ProbeError(String),

    #[error("Project file error: {0}")]
    ProjectError(String),

    #[error("General error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, CutError>;
