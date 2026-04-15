# Oberon Architecture

## Overview

Oberon is a multi-layered AI agent runtime designed for local execution. Each layer is decoupled and independently testable.

### Layer 1: Agent Core
- ReAct loop (Thought → Action → Observation)
- Self-reflection and error recovery
- Coordinates planner, memory, and execution engine

### Layer 2: Planner
- Converts natural language into structured execution plans
- Uses LLM with strict JSON schema prompting
- Supports dynamic replanning on failure

### Layer 3: Execution Engine
- Validates JSON actions against schema
- Dispatches to tool registry
- Returns observations to agent

### Layer 4: Tooling Layer
- Plugin system using Rust traits
- Built-in tools: file I/O, search (regex), git diff/commit, refactoring
- Each tool validates input and returns structured output

### Layer 5: Safety & Sandbox
- Path allowlist (defaults to project root and home)
- Dry-run mode for preview
- Destructive action confirmation
- Policy enforcement for dangerous commands

### Layer 6: LLM Integration
- Abstract provider trait
- Ollama implementation with streaming support
- System prompts enforce JSON output

### Layer 7: Memory System
- Short-term: sliding window of conversation + observations
- Long-term: local JSON storage of session summaries

### Layer 8: Observability
- Structured logging via `tracing`
- JSON output for log aggregation
- Debug mode with step-by-step tracing

### Layer 9: Git Intelligence
- Diff analysis
- Commit message generation
- Safe refactoring with validation

### Layer 10: CLI Interface
- Natural language commands
- Interactive REPL
- Streaming output