export function isMacPlatform(): boolean {
  return (
    typeof navigator !== "undefined" &&
    /Mac|iPhone|iPad/.test(navigator.platform)
  );
}

export function modLabel(letter: string): string {
  const key = letter.length === 1 ? letter.toUpperCase() : letter;
  return isMacPlatform() ? `⌘${key}` : `Ctrl+${key}`;
}

export function hasModifier(event: KeyboardEvent): boolean {
  return event.metaKey || event.ctrlKey;
}

export function modKey(event: KeyboardEvent): string | null {
  if (!hasModifier(event)) return null;
  return event.key.toLowerCase();
}
