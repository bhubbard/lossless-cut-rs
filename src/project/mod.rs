use crate::error::Result;
use crate::timecode::{format_timecode, parse_timecode};
use crate::types::{ProjectFile, Segment};
use std::path::Path;

/// Loads an official LosslessCut `.llc` JSON project file.
pub fn load_llc_project(path: &Path) -> Result<ProjectFile> {
    let content = std::fs::read_to_string(path)?;
    let proj: ProjectFile = serde_json::from_str(&content)?;
    Ok(proj)
}

/// Saves segments to an official LosslessCut `.llc` JSON project file.
pub fn save_llc_project(path: &Path, project: &ProjectFile) -> Result<()> {
    let json = serde_json::to_string_pretty(project)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Loads a segment list from a CSV file. Supported headers: `start,end,label` or raw timestamps.
pub fn load_csv_segments(path: &Path) -> Result<Vec<Segment>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let headers = reader.headers()?.clone();
    let label_idx = headers.iter().position(|h| h.eq_ignore_ascii_case("label"));

    let mut segments = Vec::new();
    for result in reader.records() {
        let record = result?;
        if record.len() >= 2 {
            let start = parse_timecode(&record[0])?;
            let end = parse_timecode(&record[1])?;
            let mut seg = Segment::new(start, end);

            let label_val = if let Some(idx) = label_idx {
                record.get(idx)
            } else if record.len() >= 4 {
                record.get(3)
            } else if record.len() == 3 {
                record.get(2)
            } else {
                None
            };

            if let Some(label) = label_val {
                if !label.trim().is_empty() {
                    seg = seg.with_label(label);
                }
            }
            segments.push(seg);
        }
    }
    Ok(segments)
}

/// Exports segments to a standard CSV file.
pub fn save_csv_segments(path: &Path, segments: &[Segment]) -> Result<()> {
    let mut writer = csv::WriterBuilder::new().from_path(path)?;
    writer.write_record(["start", "end", "duration", "label"])?;

    for s in segments {
        writer.write_record([
            format_timecode(s.start),
            format_timecode(s.end),
            format!("{:.3}", s.duration()),
            s.label.clone().unwrap_or_default(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_llc_project_roundtrip() {
        let segs = vec![
            Segment::new(0.0, 15.5).with_label("Intro"),
            Segment::new(30.0, 45.2).with_label("Highlight"),
        ];
        let proj = ProjectFile::new("movie.mp4", segs);

        let tmp = NamedTempFile::new().unwrap();
        save_llc_project(tmp.path(), &proj).expect("save llc");

        let loaded = load_llc_project(tmp.path()).expect("load llc");
        assert_eq!(loaded.media_file_name, "movie.mp4");
        assert_eq!(loaded.cut_segments.len(), 2);
        assert_eq!(loaded.cut_segments[0].label, Some("Intro".to_string()));
        assert_eq!(loaded.cut_segments[1].end, 45.2);
    }

    #[test]
    fn test_csv_roundtrip() {
        let segs = vec![
            Segment::new(10.0, 20.0).with_label("Scene 1"),
            Segment::new(60.0, 120.0).with_label("Scene 2"),
        ];

        let tmp = NamedTempFile::new().unwrap();
        save_csv_segments(tmp.path(), &segs).expect("save csv");

        let loaded = load_csv_segments(tmp.path()).expect("load csv");
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].start, 10.0);
        assert_eq!(loaded[0].end, 20.0);
        assert_eq!(loaded[0].label, Some("Scene 1".to_string()));
    }
}
