use crate::error::{CutError, Result};

/// Parses a timecode string (e.g. "01:23:45.678", "12:34.5", "45", "123.456") into seconds.
pub fn parse_timecode(s: &str) -> Result<f64> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(0.0);
    }

    // Direct decimal float check
    if let Ok(secs) = s.parse::<f64>() {
        if secs < 0.0 {
            return Err(CutError::InvalidTimecode(s.to_string()));
        }
        return Ok(secs);
    }

    // HH:MM:SS.mmm or MM:SS.mmm
    let parts: Vec<&str> = s.split(':').collect();
    match parts.len() {
        2 => {
            // MM:SS
            let mins: f64 = parts[0]
                .parse()
                .map_err(|_| CutError::InvalidTimecode(s.to_string()))?;
            let secs: f64 = parts[1]
                .parse()
                .map_err(|_| CutError::InvalidTimecode(s.to_string()))?;
            Ok(mins * 60.0 + secs)
        }
        3 => {
            // HH:MM:SS
            let hrs: f64 = parts[0]
                .parse()
                .map_err(|_| CutError::InvalidTimecode(s.to_string()))?;
            let mins: f64 = parts[1]
                .parse()
                .map_err(|_| CutError::InvalidTimecode(s.to_string()))?;
            let secs: f64 = parts[2]
                .parse()
                .map_err(|_| CutError::InvalidTimecode(s.to_string()))?;
            Ok(hrs * 3600.0 + mins * 60.0 + secs)
        }
        _ => Err(CutError::InvalidTimecode(s.to_string())),
    }
}

/// Formats seconds into standard FFmpeg timecode format "HH:MM:SS.mmm".
pub fn format_timecode(seconds: f64) -> String {
    let total_secs = seconds.max(0.0);
    let hrs = (total_secs / 3600.0) as u32;
    let mins = ((total_secs % 3600.0) / 60.0) as u32;
    let secs = (total_secs % 60.0) as u32;
    let millis = ((total_secs.fract()) * 1000.0).round() as u32;
    format!("{:02}:{:02}:{:02}.{:03}", hrs, mins, secs, millis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timecode() {
        assert_eq!(parse_timecode("0.0").unwrap(), 0.0);
        assert_eq!(parse_timecode("45.5").unwrap(), 45.5);
        assert_eq!(parse_timecode("01:30").unwrap(), 90.0);
        assert_eq!(parse_timecode("01:02:03.500").unwrap(), 3723.5);
        assert_eq!(parse_timecode("00:00:15").unwrap(), 15.0);
    }

    #[test]
    fn test_format_timecode() {
        assert_eq!(format_timecode(0.0), "00:00:00.000");
        assert_eq!(format_timecode(90.0), "00:01:30.000");
        assert_eq!(format_timecode(3723.5), "01:02:03.500");
    }
}
