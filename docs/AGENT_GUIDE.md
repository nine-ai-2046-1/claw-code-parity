# Claw Code — AI Agent Integration Guide

> For AI Agents (OpenCode, Kiro CLI, Gemini CLI, Codex, etc.) that need to invoke Claw as a subprocess or understand its interface.

---

## Invocation

### Non-interactive prompt (recommended for agents)
```bash
claw --output-format json prompt "<your prompt>"
```

### With provider override
```bash
OPENAI_API_KEY=<key> OPENAI_BASE_URL=<url> \
  claw --provider gemini --model gemini-2.5-flash \
  --output-format json prompt "<prompt>"
```

### Output format
**Success** (`--output-format json`):
```json
{
  "message": "AI response text",
  "model": "gemini-2.5-flash",
  "iterations": 1,
  "tool_uses": [],
  "tool_results": [],
  "usage": {
    "input_tokens": 120,
    "output_tokens": 85,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0
  }
}
```

**Failure**: exits with non-zero code, error on stderr.

---

## Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `ANTHROPIC_API_KEY` | Anthropic auth | `sk-ant-xxx` |
| `OPENAI_API_KEY` | OpenAI-compat auth | `gsk_xxx` (Groq) |
| `OPENAI_BASE_URL` | OpenAI-compat endpoint | `https://api.groq.com/openai/v1` |
| `XAI_API_KEY` | xAI auth | `xai-xxx` |
| `CLAW_PROVIDER` | Default provider | `gemini` |
| `CLAW_MAX_TOKENS` | Override max tokens | `8000` |
| `CLAW_DEBUG_SSE=1` | Debug streaming | — |

---

## Slash Commands via `--resume`

Agents can run slash commands on a saved session without entering REPL:

```bash
# Run /status on a session
claw --resume .claw/sessions/session-xxx.jsonl /status

# Run /compact
claw --resume .claw/sessions/session-xxx.jsonl /compact

# Export session
claw --resume .claw/sessions/session-xxx.jsonl /export output.txt
```

Supported resume commands: `help`, `status`, `compact`, `clear`, `cost`, `config`, `memory`, `init`, `diff`, `version`, `export`, `agents`, `skills`

> Note: `/status` outputs plain text only. `--output-format json` is not supported in `--resume` mode.

---

## AI-Powered Slash Commands (Agent Use)

These commands are designed to be called by agents. All support `--output-format json`.

### `/skillify [output-path]`
Generate a SKILL.md from the current conversation.
```bash
# Via REPL pipe
printf "conversation context\n/skillify /tmp/skill.md\n/exit\n" | claw --provider gemini ...
```

### `/simplify [glob-pattern]`
3-dimension code review (Reuse · Quality · Efficiency).
```bash
printf "/simplify src/**/*.rs\n/exit\n" | claw ...
```

### `/dream`
Distil conversation into `.claw/CLAUDE.md` memory.
```bash
printf "important context\n/dream\n/exit\n" | claw ...
```

### `/batch <task> --yes`
Break task into subtasks and execute. Use `--yes` to skip confirmation.
```bash
printf "/batch refactor auth module --yes\n/exit\n" | claw ...
```

### `/kairos <task>`
Coordinator + Workers architecture for complex tasks.
```bash
printf "/kairos analyse security vulnerabilities\n/exit\n" | claw ...
```

---

## Permission Modes

| Mode | Tools allowed | Use case |
|------|--------------|---------|
| `read-only` | Read/search only | Safe exploration |
| `workspace-write` | Edit files in workspace | Normal development |
| `danger-full-access` | Unrestricted | Full automation |

```bash
claw --permission-mode workspace-write --output-format json prompt "fix the bug in src/auth.rs"
```

---

## Session Management for Agents

```bash
# Sessions stored in .claw/sessions/
ls .claw/sessions/

# Resume latest session
claw --resume latest /status

# Get session info as JSON
claw --resume .claw/sessions/session-xxx.jsonl /status --output-format json
```

---

## Tool Control

```bash
# Restrict to specific tools
claw --allowedTools read_file,grep_search prompt "find all TODO comments"

# Disable all tools
claw --allowedTools "" prompt "explain this concept"
```

---

## Error Handling

| Exit code | Meaning |
|-----------|---------|
| `0` | Success |
| `1` | Error (check stderr) |

Common errors:
- `missing Anthropic credentials` → Set `ANTHROPIC_API_KEY`
- `unsupported provider 'xxx'` → Check `--provider` value
- `api returned 429` → Rate limit, retry after delay
- `api returned 401` → Invalid API key

---

## Integration Pattern

```python
import subprocess, json, os

def call_claw(prompt: str, provider: str = "anthropic") -> dict:
    env = {**os.environ}
    result = subprocess.run(
        ["claw", "--provider", provider, "--output-format", "json", "prompt", prompt],
        capture_output=True, text=True, env=env
    )
    if result.returncode != 0:
        raise RuntimeError(f"claw failed: {result.stderr}")
    return json.loads(result.stdout)

response = call_claw("summarise src/main.rs")
print(response["message"])
```
