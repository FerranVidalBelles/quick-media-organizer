use crate::models::{MediaItem, MediaKind, SortMode};
use crate::path_util::IGNORED_FOLDER_NAMES;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "webp", "gif", "heic", "heif", "bmp", "tiff", "tif",
];
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "m4v", "avi", "mkv", "3gp"];
const IGNORED_DIRS: &[&str] = IGNORED_FOLDER_NAMES;

pub fn is_media_extension(ext: &str) -> bool {
    let ext = ext.to_ascii_lowercase();
    IMAGE_EXTENSIONS.contains(&ext.as_str()) || VIDEO_EXTENSIONS.contains(&ext.as_str())
}

pub fn is_video_extension(ext: &str) -> bool {
    VIDEO_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str())
}

pub fn list_subfolders(root: &Path) -> Vec<String> {
    let mut folders = Vec::new();

    if !root.is_dir() {
        return folders;
    }

    for entry in WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|name| IGNORED_DIRS.contains(&name))
        {
            continue;
        }

        if let Ok(relative) = path.strip_prefix(root) {
            folders.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }

    folders.sort();
    folders.dedup();
    folders
}

pub fn count_root_subfolder_media(root: &Path) -> usize {
    let mut count = 0usize;
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|name| IGNORED_DIRS.contains(&name))
            {
                continue;
            }
            if let Ok(sub) = fs::read_dir(&path) {
                for file in sub.filter_map(Result::ok) {
                    if file.path().is_file() && is_supported_file(&file.path()) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn scan_folder(root: &Path, recursive: bool) -> Result<Vec<MediaItem>, String> {
    let mut files: Vec<PathBuf> = Vec::new();

    if recursive {
        for entry in WalkDir::new(root)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if is_ignored_path(root, path) {
                continue;
            }

            if is_supported_file(path) {
                files.push(path.to_path_buf());
            }
        }
    } else if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && is_supported_file(&path) {
                files.push(path);
            }
        }
    }

    let mut grouped = group_live_photos(&files);
    sort_items(&mut grouped, SortMode::ExifDate);
    Ok(grouped)
}

fn is_ignored_path(root: &Path, path: &Path) -> bool {
    if let Ok(relative) = path.strip_prefix(root) {
        for component in relative.components() {
            if let std::path::Component::Normal(name) = component {
                if let Some(name) = name.to_str() {
                    if IGNORED_DIRS.contains(&name) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn is_supported_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(is_media_extension)
}

fn group_live_photos(files: &[PathBuf]) -> Vec<MediaItem> {
    let mut by_stem: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for path in files {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        by_stem.entry(stem).or_default().push(path.clone());
    }

    let mut used: HashSet<String> = HashSet::new();
    let mut items = Vec::new();

    for path in files {
        let id = path.to_string_lossy().to_string();
        if used.contains(&id) {
            continue;
        }

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        if let Some(group) = by_stem.get(&stem) {
            if let Some(pair) = detect_live_photo_pair(group) {
                for p in &pair {
                    used.insert(p.to_string_lossy().to_string());
                }
                if group.len() > pair.len() {
                    // Additional files with same stem stay as separate items.
                }
                items.push(build_media_item(&pair, MediaKind::LivePhoto));
                continue;
            }
        }

        used.insert(id);
        items.push(build_media_item(&[path.clone()], MediaKind::Single));
    }

    items
}

fn detect_live_photo_pair(group: &[PathBuf]) -> Option<Vec<PathBuf>> {
    if group.len() < 2 {
        return None;
    }

    let mut image: Option<PathBuf> = None;
    let mut video: Option<PathBuf> = None;

    for path in group {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        if is_video_extension(&ext) {
            video = Some(path.clone());
        } else if IMAGE_EXTENSIONS.contains(&ext.as_str()) {
            image = Some(path.clone());
        }
    }

    match (image, video) {
        (Some(img), Some(vid)) => Some(vec![img, vid]),
        _ => None,
    }
}

fn build_media_item(paths: &[PathBuf], kind: MediaKind) -> MediaItem {
    let primary = &paths[0];
    let file_name = primary
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string();
    let extension = primary
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let meta = read_file_metadata(primary);
    let size_bytes = paths
        .iter()
        .filter_map(|p| fs::metadata(p).ok())
        .map(|m| m.len())
        .sum();

    MediaItem {
        id: primary.to_string_lossy().to_string(),
        paths: paths.iter().map(|p| p.to_string_lossy().to_string()).collect(),
        file_name,
        extension,
        exif_date: meta.exif_date,
        modified_at: meta.modified_at,
        size_bytes,
        is_video: is_video_extension(
            primary
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or_default(),
        ),
        kind,
        width: meta.width,
        height: meta.height,
    }
}

struct FileMeta {
    exif_date: Option<String>,
    modified_at: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

fn read_file_metadata(path: &Path) -> FileMeta {
    let modified_at = fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .map(|t| {
            DateTime::<Utc>::from(t)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        });

    let mut exif_date = None;
    let mut width = None;
    let mut height = None;

    if let Ok(file) = fs::File::open(path) {
        if let Ok(exif) = exif::Reader::new().read_from_container(&mut std::io::BufReader::new(file))
        {
            if let Some(field) = exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY) {
                exif_date = Some(field.display_value().to_string());
            }
            if let Some(field) = exif.get_field(exif::Tag::PixelXDimension, exif::In::PRIMARY) {
                width = field.value.get_uint(0).map(|v| v as u32);
            }
            if let Some(field) = exif.get_field(exif::Tag::PixelYDimension, exif::In::PRIMARY) {
                height = field.value.get_uint(0).map(|v| v as u32);
            }
        }
    }

    FileMeta {
        exif_date,
        modified_at,
        width,
        height,
    }
}

pub fn sort_items(items: &mut [MediaItem], mode: SortMode) {
    items.sort_by(|a, b| match mode {
        SortMode::FileName => a.file_name.to_lowercase().cmp(&b.file_name.to_lowercase()),
        SortMode::ModifiedDate => a
            .modified_at
            .cmp(&b.modified_at)
            .then(a.file_name.cmp(&b.file_name)),
        SortMode::ExifDate => a
            .exif_date
            .cmp(&b.exif_date)
            .then(a.file_name.cmp(&b.file_name)),
    });
}

pub fn parse_sortable_date(value: &Option<String>) -> Option<NaiveDateTime> {
    let value = value.as_ref()?;
    NaiveDateTime::parse_from_str(value, "%Y:%m:%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S"))
        .ok()
}
