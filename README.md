# Oberon

**Local-first AI Agent Runtime for Developers**

Oberon is a production-grade execution engine that allows AI agents to understand developer intent, plan multi-step actions, and safely interact with your local system. It’s not a CLI tool—it’s infrastructure for autonomous development workflows.

## Features

- **ReAct Loop** with self-reflection and error recovery
- **Planner** that decomposes complex tasks
- **Extensible Tool System** (files, git, search, refactoring)
- **Safety Sandbox** with allowlists, dry-run, and confirmations
- **Git Intelligence** for diff analysis and commit suggestions
- **Local LLM** via Ollama (provider-agnostic)
- **Memory** (short-term and long-term)
- **Observability** with structured logging

## Quick Start

```bash
# Clone the repository
git clone https://github.com/oberon-ai/oberon.git
cd oberon

# Build
cargo build --release

# Run a task
./target/release/oberon run "List all Rust files and count lines of code"

# Interactive mode
./target/release/oberon