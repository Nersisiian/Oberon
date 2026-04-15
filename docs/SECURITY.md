# Security Considerations

## Threat Model
Oberon executes LLM-generated actions on a developer's machine. The primary risks are:
- Unintended file modifications/deletions
- Execution of malicious shell commands
- Exfiltration of sensitive data

## Mitigations

### File System Allowlist
By default, Oberon only allows access to:
- Current working directory
- User's home directory
- Temporary directories

All paths are canonicalized before checking.

### Dry-Run Mode
`--dry-run` flag previews all actions without executing writes.

### Confirmation Prompts
Destructive actions (write, delete, git commit) require user confirmation unless disabled.

### Sandboxed Execution
Tools run with the same privileges as the user but cannot escape the allowlist.

### Input Validation
All tool inputs are validated against schemas before execution.

### Logging
All actions are logged with full context for auditability.

## Best Practices
- Run Oberon in a dedicated development container for sensitive projects
- Review plans before execution in dry-run mode
- Keep your Ollama model updated