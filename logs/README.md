# App error log

While developing, the app writes errors to:

```
logs/app-errors.jsonl
```

Each line is one JSON error (timestamp, source, message, context, stack).

Production builds also mirror to the app data folder.

When bugs are fixed, this file is cleared so only new issues remain.
