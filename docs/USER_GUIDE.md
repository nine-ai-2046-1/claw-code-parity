# Claw Code — User Guide

> 🦞 An AI coding assistant that runs in your terminal, supports multiple AI providers, and comes with powerful built-in tools.

---

## Installation

### Prerequisites
- Rust toolchain ([rustup.rs](https://rustup.rs))
- Git

### Build from source
```bash
git clone https://github.com/nine-ai-2046-1/claw-code-parity.git
cd claw-code-parity/rust
cargo build --release -p rusty-claude-cli

# Add to PATH
export PATH=$PATH:$(pwd)/target/release
```

---

## Quick Start

```bash
# Set your API key
export ANTHROPIC_API_KEY=sk-ant-xxx

# Start interactive REPL
claw

# One-shot prompt
claw "explain this codebase"

# Use Gemini instead
export OPENAI_API_KEY=your_gemini_key
export OPENAI_BASE_URL=https://generativelanguage.googleapis.com/v1beta/openai/
claw --provider gemini --model gemini-2.5-flash "hello"
```

---

## Supported Providers

| Provider | `--provider` value | Required env vars |
|----------|-------------------|-------------------|
| Anthropic (default) | `anthropic` / `claude` | `ANTHROPIC_API_KEY` |
| Gemini | `gemini` | `OPENAI_API_KEY` + `OPENAI_BASE_URL` |
| Groq | `groq` | `OPENAI_API_KEY` + `OPENAI_BASE_URL` |
| OpenAI | `openai` | `OPENAI_API_KEY` |
| xAI Grok | `xai` / `grok` | `XAI_API_KEY` |
| Poe | `poe` | `OPENAI_API_KEY` + `OPENAI_BASE_URL` |
| OpenRouter | `openrouter` | `OPENAI_API_KEY` + `OPENAI_BASE_URL` |

```bash
# Session-level default provider
export CLAW_PROVIDER=gemini
```

---

## CLI Flags

| Flag | Description |
|------|-------------|
| `--provider <name>` | Override AI provider |
| `--model <name>` | Override model (e.g. `opus`, `gemini-2.5-flash`) |
| `--output-format json` | JSON output (for scripting) |
| `--permission-mode read-only` | Restrict tool access |
| `CLAW_MAX_TOKENS=8000` | Override max tokens |
| `CLAW_DEBUG_SSE=1` | Debug streaming output |

---

## Built-in Tools

When in REPL or prompt mode, Claw can use these tools automatically:

| Tool | What it does |
|------|-------------|
| `bash` | Run shell commands |
| `read_file` | Read files |
| `write_file` | Write / create files |
| `edit_file` | Edit files (patch) |
| `glob_search` | Find files by pattern |
| `grep_search` | Search file contents |
| `web_search` | Search the web |

---

## Slash Commands

Type these in the REPL (or pass via `--resume`):

### Navigation & Info
| Command | Description |
|---------|-------------|
| `/help` | Show all commands |
| `/status` | Token usage, model, session info |
| `/version` | Version info |
| `/diff` | Show current git diff |

### Session Management
| Command | Description |
|---------|-------------|
| `/clear --confirm` | Start fresh session |
| `/compact` | Compress conversation history |
| `/resume <path>` | Resume a saved session |
| `/session list` | List saved sessions |
| `/export [file]` | Export conversation |

### Git & Code
| Command | Description |
|---------|-------------|
| `/commit` | AI-generated git commit message |
| `/pr [context]` | Draft a pull request |
| `/issue [context]` | Draft a GitHub issue |

### AI-Powered Features
| Command | Description | Example |
|---------|-------------|---------|
| `/skillify [output]` | Generate SKILL.md from conversation | `/skillify skills/my-skill.md` |
| `/simplify [glob]` | 3-dimension code review | `/simplify src/**/*.rs` |
| `/dream` | Distil conversation into memory | `/dream` |
| `/buddy` | Show your companion | `/buddy` |
| `/batch <task> [--yes]` | Break task into subtasks & execute. Use `--yes` to skip confirmation prompt | `/batch --yes "refactor auth module"` |
| `/kairos <task>` | Coordinator + Workers architecture | `/kairos "analyse security issues"` |
| `/bughunter [scope]` | Find bugs in codebase | `/bughunter src/` |
| `/ultraplan [task]` | Deep multi-step execution plan | `/ultraplan "migrate to async"` |

### Discovery
| Command | Description |
|---------|-------------|
| `/agents` | List available agents |
| `/skills` | List available skills |
| `/memory` | Show loaded memory files |
| `/teleport <symbol>` | Jump to symbol/file |

---

## Memory System

Claw automatically loads `.claw/CLAUDE.md` (and `.claw/instructions.md`) from your project directory as persistent instructions.

```bash
# Manually add memory
echo "Always use Cantonese in responses" >> .claw/CLAUDE.md

# Or use /dream to auto-distil from conversation
/dream
```

---

## Use Cases

### 1. Code Review
```bash
claw
> /simplify src/main.rs
# → Reuse · Quality · Efficiency review
```

### 2. Automated Task Execution
```bash
claw
> /batch "add unit tests to all public functions in src/" --yes
```

### 3. Knowledge Capture
```bash
claw
> We just decided to use PostgreSQL for all new services
> /dream
# → Saves to .claw/CLAUDE.md, loaded next session
```

### 4. Multi-Provider Comparison
```bash
# Try same prompt with different providers
OPENAI_API_KEY=gemini_key OPENAI_BASE_URL=... claw --provider gemini "explain ownership"
OPENAI_API_KEY=groq_key OPENAI_BASE_URL=... claw --provider groq --model llama-3.1-8b-instant "explain ownership"
```

### 5. Scripting / CI
```bash
# JSON output for parsing
claw --output-format json prompt "summarise changes in src/" | jq '.message'
```

---

## Tips

- **Permissions**: Default is `danger-full-access`. Use `--permission-mode read-only` for safer exploration.
- **Sessions**: Saved in `.claw/sessions/`. Use `/resume latest` to jump back.
- **Skills**: Place `SKILL.md` files in `.claw/skills/` to give Claw reusable workflows.
- **Plugins**: Extend tools via `.claw/plugins/`.
