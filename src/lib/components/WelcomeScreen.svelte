<script lang="ts">
  import Kbd from "./Kbd.svelte";
  import SupportBlock from "./SupportBlock.svelte";
  import { t, type Locale } from "../i18n";
  import { modLabel } from "../shortcuts";

  let {
    locale,
    dontShowAgain = $bindable(false),
    hideSupport = false,
    onStart,
  }: {
    locale: Locale;
    dontShowAgain?: boolean;
    hideSupport?: boolean;
    onStart: () => void;
  } = $props();
</script>

<div class="welcome-screen">
  <div class="welcome-card">
    <h1>{t(locale, "welcomeTitle")}</h1>
    <div class="welcome-steps">
      <div class="welcome-step">1. {t(locale, "welcomeStep1")}</div>
      <div class="welcome-step">
        2. {t(locale, "welcomeStep2")}
        <div class="shortcut-row" style="margin-top:0.75rem;">
          <Kbd label="Enter" text={t(locale, "shortcuts.enter")} />
          <Kbd label={modLabel("F")} text={t(locale, "shortcuts.folder")} />
          <Kbd label={modLabel("D")} text={t(locale, "shortcuts.delete")} danger />
          <Kbd label="Space" text={t(locale, "shortcuts.space")} />
        </div>
      </div>
      <div class="welcome-step">3. {t(locale, "welcomeStep3")}</div>
    </div>

    <label class="checkbox-row">
      <input type="checkbox" bind:checked={dontShowAgain} />
      {t(locale, "welcomeDontShow")}
    </label>

    <div class="modal-actions">
      <button class="primary-btn" onclick={onStart}>{t(locale, "welcomeStart")}</button>
    </div>

    {#if !hideSupport}
      <SupportBlock {locale} compact />
    {/if}
  </div>
</div>
