<script lang="ts">
  import Kbd from "./Kbd.svelte";
  import { format, t, type Locale } from "../i18n";
  import { modLabel } from "../shortcuts";

  let {
    locale,
    current,
    total,
    activeKey = "",
    disabled = false,
    vertical = false,
    onSave,
    onFolder,
    onDelete,
    onSkip,
    onPrev,
    onNext,
    onUndo,
    onInfo,
    onOptions,
    onHelp,
    pendingTrim = false,
  }: {
    locale: Locale;
    current: number;
    total: number;
    activeKey?: string;
    disabled?: boolean;
    vertical?: boolean;
    onSave: () => void;
    onFolder: () => void;
    onDelete: () => void;
    onSkip: () => void;
    onPrev: () => void;
    onNext: () => void;
    onUndo: () => void;
    onInfo: () => void;
    onOptions: () => void;
    onHelp: () => void;
    pendingTrim?: boolean;
  } = $props();

  const folderKey = modLabel("F");
  const deleteKey = modLabel("D");
  const infoKey = modLabel("M");
  const optionsKey = modLabel("O");
  const undoKey = modLabel("Z");
</script>

<div class="shortcut-bar" class:shortcut-bar-vertical={vertical}>
  <div class="shortcut-row">
    <Kbd
      label="Enter"
      text={pendingTrim ? t(locale, "shortcuts.enterTrimSave") : t(locale, "shortcuts.enter")}
      active={activeKey === "Enter"}
      {disabled}
      onclick={onSave}
    />
    <Kbd
      label={folderKey}
      text={t(locale, "shortcuts.folder")}
      active={activeKey === folderKey}
      {disabled}
      onclick={onFolder}
    />
    <Kbd
      label={deleteKey}
      text={t(locale, "shortcuts.delete")}
      danger
      active={activeKey === deleteKey || activeKey === "Delete"}
      {disabled}
      onclick={onDelete}
    />
    <Kbd
      label="Space"
      text={t(locale, "shortcuts.space")}
      active={activeKey === " "}
      {disabled}
      onclick={onSkip}
    />
    <Kbd
      label="←"
      text={t(locale, "shortcuts.prev")}
      active={activeKey === "ArrowLeft"}
      {disabled}
      onclick={onPrev}
    />
    <Kbd
      label="→"
      text={t(locale, "shortcuts.next")}
      active={activeKey === "ArrowRight"}
      {disabled}
      onclick={onNext}
    />
    <Kbd
      label={undoKey}
      text={t(locale, "shortcuts.undo")}
      active={activeKey === "Undo" || activeKey === undoKey}
      {disabled}
      onclick={onUndo}
    />
    <Kbd
      label={infoKey}
      text={t(locale, "shortcuts.info")}
      active={activeKey === infoKey}
      {disabled}
      onclick={onInfo}
    />
    <Kbd
      label={optionsKey}
      text={t(locale, "shortcuts.options")}
      active={activeKey === optionsKey}
      {disabled}
      onclick={onOptions}
    />
    <Kbd
      label="?"
      text={t(locale, "shortcuts.help")}
      active={activeKey === "?"}
      {disabled}
      onclick={onHelp}
    />
    <span class="shortcut-progress">
      {format(locale, "progress", {
        current: total === 0 ? 0 : current + 1,
        total,
      })}
    </span>
  </div>
</div>
