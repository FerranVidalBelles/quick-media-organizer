use std::path::{Path, PathBuf};
use std::process::Command;

pub struct FfmpegTools {
    ffmpeg: PathBuf,
    ffprobe: PathBuf,
}

impl FfmpegTools {
    pub fn locate() -> Result<Self, String> {
        let ffmpeg = find_binary("ffmpeg")?;
        let ffprobe = find_binary("ffprobe")?;
        Ok(Self { ffmpeg, ffprobe })
    }

    pub fn probe_duration(&self, path: &Path) -> Result<f64, String> {
        let output = Command::new(&self.ffprobe)
            .args([
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
            ])
            .arg(path)
            .output()
            .map_err(|e| format!("Failed to run ffprobe: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "ffprobe failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let text = String::from_utf8_lossy(&output.stdout);
        text.trim()
            .parse::<f64>()
            .map_err(|_| "Could not parse video duration.".into())
    }

    pub fn trim_lossless(
        &self,
        input: &Path,
        output: &Path,
        start_secs: f64,
        end_secs: f64,
    ) -> Result<(), String> {
        if end_secs <= start_secs + 0.05 {
            return Err("Trim range is too short.".into());
        }

        let output = Command::new(&self.ffmpeg)
            .arg("-y")
            .arg("-i")
            .arg(input)
            .arg("-ss")
            .arg(format!("{start_secs:.3}"))
            .arg("-to")
            .arg(format!("{end_secs:.3}"))
            .arg("-c")
            .arg("copy")
            .arg("-map")
            .arg("0")
            .arg("-avoid_negative_ts")
            .arg("make_zero")
            .arg(output)
            .output()
            .map_err(|e| format!("Failed to run ffmpeg: {e}"))?;

        if output.status.success() {
            return Ok(());
        }

        Err(format!(
            "ffmpeg trim failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn find_binary(name: &str) -> Result<PathBuf, String> {
    let candidates = if cfg!(target_os = "macos") {
        vec![
            name.to_string(),
            format!("/opt/homebrew/bin/{name}"),
            format!("/usr/local/bin/{name}"),
        ]
    } else if cfg!(target_os = "windows") {
        vec![
            name.to_string(),
            format!("{name}.exe"),
            format!(r"C:\ffmpeg\bin\{name}.exe"),
        ]
    } else {
        vec![name.to_string(), format!("/usr/bin/{name}")]
    };

    for candidate in candidates {
        if Command::new(&candidate)
            .arg("-version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Ok(PathBuf::from(candidate));
        }
    }

    Err(format!(
        "FFmpeg not found ({name}). Install it to trim videos losslessly (e.g. brew install ffmpeg)."
    ))
}

pub fn trim_backup_path(folder: &Path, video_path: &Path) -> PathBuf {
    let session = folder
        .join(crate::path_util::APP_FOLDER_NAME)
        .join("trim-backups");
    let stem = video_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("video");
    let ext = video_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp4");
    let stamp = chrono::Local::now().format("%Y%m%d_%H%M%S_%f");
    session.join(format!("{stem}_{stamp}.{ext}"))
}
