# Oberon Session Example

## Task: Find all TODOs and generate tasks

```bash
$ oberon run "find all TODOs and generate tasks"
```

### Output

```
ℹ Starting agent execution
ℹ Plan created:
  1. Search code for TODO comments
  2. Parse results and create task list
  3. Write tasks to TODO.md

✓ Found 5 TODO comments in 3 files
✓ Generated tasks:
  - [ ] Refactor error handling in auth.rs
  - [ ] Add unit tests for user module
  - [ ] Update documentation for API
  - [ ] Optimize database queries
  - [ ] Remove deprecated functions
✓ Written to TODO.md

All done! Tasks generated successfully.
```

## Interactive Mode

```bash
$ oberon
> refactor src/auth.rs to use async/await
Thinking...
✓ Read src/auth.rs
✓ Refactored successfully
✓ Wrote changes to src/auth.rs

> git commit with message "refactor(auth): convert to async/await"
✓ Committed with message: refactor(auth): convert to async/await
```