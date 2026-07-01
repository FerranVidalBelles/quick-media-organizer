<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { t, type Locale } from "../i18n";
  import type { MediaItem } from "../types";

  let {
    locale,
    item,
    videoRef = $bindable<HTMLVideoElement | null>(null),
    demoMode = false,
  }: {
    locale: Locale;
    item: MediaItem | null | undefined;
    videoRef?: HTMLVideoElement | null;
    demoMode?: boolean;
  } = $props();

  const previewPath = $derived(item?.paths[0] ?? "");
  const assetUrl = $derived(
    demoMode && previewPath
      ? previewPath
      : previewPath
        ? `${convertFileSrc(previewPath)}?v=${encodeURIComponent(item?.id ?? previewPath)}`
        : "",
  );
</script>

<div class="preview-panel">
  {#if item}
    {#key item.id}
      {#if item.kind === "live_photo"}
        <span class="live-badge">{t(locale, "livePhoto")}</span>
      {/if}

      <div class="preview-stage">
        {#if item.is_video}
          <video
            bind:this={videoRef}
            class="preview-media"
            src={assetUrl}
            controls
            autoplay
            muted
            playsinline
          ></video>
        {:else}
          <img class="preview-media" src={assetUrl} alt={item.file_name} />
        {/if}
      </div>
    {/key}
  {/if}
</div>
