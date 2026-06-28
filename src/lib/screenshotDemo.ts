import type { FrontendState, MediaItem } from "./types";

export type ScreenshotMode = "welcome" | "workspace" | "workspace-video";

export function getScreenshotMode(): ScreenshotMode | null {
  if (typeof window === "undefined") return null;
  const value = new URLSearchParams(window.location.search).get("screenshot");
  if (value === "welcome" || value === "workspace" || value === "workspace-video") {
    return value;
  }
  return null;
}

const demoPhotoItem: MediaItem = {
  id: "demo-sunset",
  paths: ["/demo-sunset.jpg"],
  file_name: "IMG_4521.heic",
  extension: "heic",
  exif_date: "2024-08-12T19:42:00",
  modified_at: "2024-08-12T19:42:00",
  size_bytes: 2_457_600,
  is_video: false,
  kind: "live_photo",
  width: 3024,
  height: 4032,
};

const demoVideoItem: MediaItem = {
  id: "demo-waves",
  paths: ["/demo-video.mp4"],
  file_name: "IMG_8834.mov",
  extension: "mov",
  exif_date: "2024-08-12T19:55:00",
  modified_at: "2024-08-12T19:55:00",
  size_bytes: 8_420_000,
  is_video: true,
  kind: "single",
  width: 1080,
  height: 1920,
};

function buildBaseWorkspaceState(item: MediaItem, index: number): FrontendState {
  return {
    folder_path: "/Users/demo/Phone Backup 2024",
    current_index: index,
    total: 2410,
    item,
    sort_mode: "exif_date",
    scan_recursive: false,
    rename_mode: "free",
    armed_folder: "trips/portugal/algarve/beach-holidays-2024",
    recent_folders: ["trips/portugal/algarve/beach-holidays-2024", "gym", "paperwork"],
    favorite_folders: ["trips/portugal/algarve/beach-holidays-2024"],
    existing_subfolders: [
      "gym",
      "trips",
      "trips/portugal",
      "trips/portugal/algarve",
      "trips/portugal/algarve/beach-holidays-2024",
      "paperwork",
    ],
    stats: { renamed: 412, trashed: 89, moved: 346, skipped: 1203 },
  };
}

export function buildScreenshotWorkspaceState(): FrontendState {
  return buildBaseWorkspaceState(demoPhotoItem, 846);
}

export function buildScreenshotVideoWorkspaceState(): FrontendState {
  return buildBaseWorkspaceState(demoVideoItem, 847);
}
