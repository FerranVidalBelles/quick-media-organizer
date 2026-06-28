<script lang="ts">
  let {
    label,
    text,
    danger = false,
    active = false,
    disabled = false,
    onclick,
  }: {
    label: string;
    text: string;
    danger?: boolean;
    active?: boolean;
    disabled?: boolean;
    onclick?: () => void;
  } = $props();
</script>

<button
  type="button"
  class="kbd-chip"
  class:danger
  class:active
  {disabled}
  {onclick}
  title={text}
>
  <span class="kbd-key">{label}</span>
  <span class="kbd-text">{text}</span>
</button>
