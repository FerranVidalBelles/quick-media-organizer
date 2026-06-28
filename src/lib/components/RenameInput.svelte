<script lang="ts">
  import Kbd from "./Kbd.svelte";
  import { t, type Locale } from "../i18n";
  import { modLabel } from "../shortcuts";

  let {
    locale,
    value = $bindable(""),
    armedFolder = null,
    inputRef = $bindable<HTMLInputElement | null>(null),
    compact = false,
    pendingTrim = false,
  }: {
    locale: Locale;
    value?: string;
    armedFolder?: string | null;
    inputRef?: HTMLInputElement | null;
    compact?: boolean;
    pendingTrim?: boolean;
  } = $props();
</script>

<div class="rename-row">
  <label class="field-label" for="rename-input">
    {t(locale, "nameLabel")}
  </label>
  <div class="rename-input-wrap">
    <input
      id="rename-input"
      class="rename-input"
      bind:this={inputRef}
      bind:value
      placeholder={t(locale, "namePlaceholder")}
      autocomplete="off"
      spellcheck="false"
    />
  </div>
  {#if armedFolder}
    <div class="folder-badge" title={armedFolder}>
      <span class="folder-badge-label">{t(locale, "armedFolder")}:</span>
      <span class="folder-badge-path">{armedFolder}</span>
    </div>
  {/if}
  {#if !compact}
    <div class="inline-hints">
      <Kbd
        label="Enter"
        text={pendingTrim ? t(locale, "hints.enterTrimSave") : t(locale, "hints.enter")}
      />
      <Kbd label={modLabel("F")} text={t(locale, "hints.folder")} />
      <Kbd label={modLabel("D")} text={t(locale, "hints.delete")} danger />
      <Kbd label="Space" text={t(locale, "hints.space")} />
    </div>
  {/if}
</div>
