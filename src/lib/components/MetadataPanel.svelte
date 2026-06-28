<script lang="ts">
  import { formatBytes, formatDate } from "../utils";
  import { format, t, type Locale } from "../i18n";
  import { modLabel } from "../shortcuts";
  import type { MediaItem } from "../types";

  let {
    locale,
    item,
    visible = $bindable(true),
  }: {
    locale: Locale;
    item: MediaItem | null | undefined;
    visible?: boolean;
  } = $props();

  const toggleHint = $derived(format(locale, "metadata.toggle", { key: modLabel("M") }));
</script>

{#if visible && item}
  <div class="metadata-panel">
    <div class="metadata-grid">
      <span><strong>{t(locale, "metadata.file")}:</strong> {item.file_name}</span>
      <span><strong>{t(locale, "metadata.date")}:</strong> {formatDate(item.exif_date ?? item.modified_at)}</span>
      <span><strong>{t(locale, "metadata.size")}:</strong> {formatBytes(item.size_bytes)}</span>
      {#if item.width && item.height}
        <span><strong>{t(locale, "metadata.dimensions")}:</strong> {item.width}×{item.height}</span>
      {/if}
    </div>
  </div>
{:else}
  <button type="button" class="metadata-panel metadata-toggle" onclick={() => (visible = true)}>
    {toggleHint}
  </button>
{/if}
