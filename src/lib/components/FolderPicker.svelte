<script lang="ts">
  import { t, type Locale } from "../i18n";

  let {
    locale,
    open,
    query = $bindable(""),
    selected = $bindable<string | null>(null),
    favorites,
    recent,
    existing,
    onConfirm,
    onClose,
    onToggleFavorite,
  }: {
    locale: Locale;
    open: boolean;
    query?: string;
    selected?: string | null;
    favorites: string[];
    recent: string[];
    existing: string[];
    onConfirm: () => void;
    onClose: () => void;
    onToggleFavorite: (folder: string) => void;
  } = $props();

  const suggestions = $derived.by(() => {
    const q = query.trim().toLowerCase();
    const pool = [...new Set([...recent, ...favorites, ...existing])];
    if (!q) return pool.slice(0, 12);
    return pool.filter((item) => item.toLowerCase().includes(q)).slice(0, 12);
  });

  const effectiveFolder = $derived(query.trim() || selected || "");

  function pick(folder: string) {
    selected = folder;
    query = folder;
  }

  function handleBackdropClick() {
    onClose();
  }
</script>

{#if open}
  <div
    class="modal-backdrop"
    role="presentation"
    onclick={handleBackdropClick}
    onkeydown={(event) => {
      if (event.key === "Escape") onClose();
    }}
  >
    <div
      class="modal-card"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(event) => event.stopPropagation()}
    >
      <h2>{t(locale, "folderPicker.title")}</h2>
      <label class="field-label">
        {t(locale, "folderPicker.search")}
        <input
          class="rename-input"
          bind:value={query}
          placeholder={t(locale, "folderPicker.createHint")}
        />
      </label>

      {#if favorites.length}
        <h3>{t(locale, "folderPicker.favorites")}</h3>
        <div class="folder-list">
          {#each favorites as folder}
            <button
              type="button"
              class="folder-item"
              class:selected={selected === folder || query === folder}
              onclick={() => pick(folder)}
            >
              ⭐ {folder}
            </button>
          {/each}
        </div>
      {/if}

      {#if recent.length}
        <h3>{t(locale, "folderPicker.recent")}</h3>
        <div class="folder-list">
          {#each recent as folder}
            <button
              type="button"
              class="folder-item"
              class:selected={selected === folder || query === folder}
              onclick={() => pick(folder)}
            >
              {folder}
            </button>
          {/each}
        </div>
      {/if}

      {#if suggestions.length}
        <h3>{t(locale, "folderPicker.existing")}</h3>
        <div class="folder-list">
          {#each suggestions as folder}
            <button
              type="button"
              class="folder-item"
              class:selected={selected === folder || query === folder}
              onclick={() => pick(folder)}
            >
              {folder}
            </button>
          {/each}
        </div>
      {/if}

      <div class="modal-actions">
        <button
          class="primary-btn"
          disabled={!effectiveFolder}
          onclick={() => {
            selected = query.trim() || selected;
            onConfirm();
          }}
        >
          {t(locale, "folderPicker.confirm")}
        </button>
        <button type="button" class="ghost-btn" onclick={onClose}>{t(locale, "folderPicker.cancel")}</button>
        {#if effectiveFolder}
          <button type="button" class="ghost-btn" onclick={() => onToggleFavorite(effectiveFolder)}>⭐</button>
        {/if}
      </div>
    </div>
  </div>
{/if}
