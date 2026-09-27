use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Video,
    Audio,
    Subtitle,
    Data,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

impl Segment {
    pub fn new(start: f64, end: f64) -> Self {
        Self {
            start,
            end,
            label: None,
            tags: HashMap::new(),
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn duration(&self) -> f64 {
        (self.end - self.start).max(0.0)
    }
}

/// Official LosslessCut project file format (.llc JSON).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectFile {
    pub version: u32,
    #[serde(rename = "mediaFileName")]
    pub media_file_name: String,
    #[serde(rename = "cutSegments")]
    pub cut_segments: Vec<Segment>,
}

impl ProjectFile {
    pub fn new(media_file_name: impl Into<String>, cut_segments: Vec<Segment>) -> Self {
        Self {
            version: 1,
            media_file_name: media_file_name.into(),
            cut_segments,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamInfo {
    pub index: usize,
    pub codec_name: String,
    pub codec_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub format_name: String,
    pub duration: f64,
    pub size: u64,
    pub streams: Vec<StreamInfo>,
    #[serde(default)]
    pub keyframes: Vec<f64>,
}

impl MediaMetadata {
    /// Finds the closest preceding keyframe for a given timestamp.
    pub fn nearest_prior_keyframe(&self, timestamp: f64) -> f64 {
        let mut best = 0.0;
        for &kf in &self.keyframes {
            if kf <= timestamp {
                best = kf;
            } else {
                break;
            }
        }
        best
    }

    /// Finds the closest subsequent keyframe for a given timestamp.
    pub fn nearest_subsequent_keyframe(&self, timestamp: f64) -> f64 {
        for &kf in &self.keyframes {
            if kf >= timestamp {
                return kf;
            }
        }
        self.duration
    }
}
