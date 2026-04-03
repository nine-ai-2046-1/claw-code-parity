# Claw Code — OpenClaw (龍蝦) Integration Guide

> 專為 OpenClaw Agent 設計嘅整合指引。Claw 係你嘅本地 AI 編程助手，你可以透過 subprocess 調用佢，或者直接用佢嘅 slash commands 做複雜任務。

---

## 基本調用

### 一次性 prompt（最常用）
```bash
claw --output-format json prompt "{{query}}"
```

### 指定 provider
```bash
OPENAI_API_KEY={{key}} OPENAI_BASE_URL={{url}} \
  claw --provider gemini --model gemini-2.5-flash \
  --output-format json prompt "{{query}}"
```

### 輸出格式
**成功：**
```json
{
  "message": "AI 回覆內容",
  "model": "gemini-2.5-flash",
  "iterations": 1,
  "tool_uses": [],
  "usage": { "input_tokens": 120, "output_tokens": 85 }
}
```
**失敗：** 非零 exit code，錯誤訊息喺 stderr。

---

## Provider 設定

| Provider | `--provider` | 需要嘅環境變數 |
|----------|-------------|--------------|
| Anthropic | `anthropic` | `ANTHROPIC_API_KEY` |
| Gemini | `gemini` | `OPENAI_API_KEY` + `OPENAI_BASE_URL=https://generativelanguage.googleapis.com/v1beta/openai/` |
| Groq | `groq` | `OPENAI_API_KEY` + `OPENAI_BASE_URL=https://api.groq.com/openai/v1` |
| xAI | `xai` | `XAI_API_KEY` |
| OpenAI | `openai` | `OPENAI_API_KEY` |

```bash
# Session-level 預設
export CLAW_PROVIDER=gemini
export CLAW_MAX_TOKENS=8000  # 覆蓋 max tokens（Groq 需要）
```

---

## Slash Commands 整合

透過 stdin pipe 調用 slash commands：

```bash
# /skillify — 從對話生成 SKILL.md
printf "{{conversation}}\n/skillify {{output_path}}\n/exit\n" | \
  claw --provider {{provider}} --model {{model}}

# /simplify — 三維度 code review
printf "/simplify {{glob_pattern}}\n/exit\n" | claw ...

# /dream — 蒸餾對話到記憶
printf "{{conversation}}\n/dream\n/exit\n" | claw ...

# /batch — 任務分解執行（--yes 跳過確認）
printf "/batch --yes {{task}}\n/exit\n" | claw ...

# /kairos — Coordinator + Workers 架構
printf "/kairos {{task}}\n/exit\n" | claw ...

# /buddy — 顯示電子寵物
printf "/buddy\n/exit\n" | claw ...
```

---

## 完整 Slash Command 列表

### 代碼相關
| Command | 用途 | Agent 觸發時機 |
|---------|------|--------------|
| `/skillify [path]` | 從對話生成 SKILL.md | 用戶完成一個可重複嘅工作流後 |
| `/simplify [glob]` | 三維度 code review | 用戶要求 review 代碼 |
| `/batch <task> --yes` | 任務分解並執行 | 大型任務需要分步執行 |
| `/kairos <task>` | Coordinator + Workers | 複雜任務需要多 worker 協作 |
| `/bughunter [scope]` | 搵 bug | 用戶要求 debug |
| `/ultraplan [task]` | 深度執行計劃 | 需要詳細計劃先執行 |
| `/commit` | 生成 git commit message | 用戶完成改動後 |
| `/pr [context]` | 起草 PR | 用戶要開 PR |

### 記憶相關
| Command | 用途 | Agent 觸發時機 |
|---------|------|--------------|
| `/dream` | 蒸餾對話到 `.claw/CLAUDE.md` | 對話有重要資訊需要記住 |
| `/memory` | 顯示已載入嘅記憶檔案 | 需要確認記憶狀態 |

### Session 管理
| Command | 用途 |
|---------|------|
| `/compact` | 壓縮對話歷史（token 太多時） |
| `/status` | 顯示 token 用量、model、session 資訊 |
| `/resume <path>` | 恢復已儲存嘅 session |
| `/export [file]` | 匯出對話 |

### 探索
| Command | 用途 |
|---------|------|
| `/skills` | 列出可用 skills |
| `/agents` | 列出可用 agents |
| `/teleport <symbol>` | 跳到指定 symbol/檔案 |
| `/buddy` | 顯示電子寵物 companion |

---

## 記憶系統

Claw 啟動時自動載入 `.claw/CLAUDE.md` 作為持久指令。

```bash
# 手動寫入記憶
echo "用廣東話回覆" >> .claw/CLAUDE.md

# 用 /dream 自動蒸餾
printf "重要嘅項目決定\n/dream\n/exit\n" | claw ...
```

---

## 權限模式

```bash
# 只讀（安全探索）
claw --permission-mode read-only prompt "..."

# 工作區寫入（正常開發）
claw --permission-mode workspace-write prompt "..."

# 完全存取（預設）
claw --permission-mode danger-full-access prompt "..."
```

---

## 工具控制

```bash
# 限制工具
claw --allowedTools read_file,grep_search prompt "搵所有 TODO"

# 禁用所有工具
claw --allowedTools "" prompt "解釋呢個概念"
```

---

## 錯誤處理

| Exit code | 意思 |
|-----------|------|
| `0` | 成功 |
| `1` | 失敗（睇 stderr） |

常見錯誤：
- `missing Anthropic credentials` → 設定 `ANTHROPIC_API_KEY`
- `unsupported provider 'xxx'` → 檢查 `--provider` 值
- `api returned 429` → Rate limit，等一陣再試
- `api returned 400 max_tokens` → 設定 `CLAW_MAX_TOKENS=8000`（Groq 用）

---

## 使用場景

### 場景1：代碼 Review
```bash
# OpenClaw 調用 claw 做 review
printf "/simplify src/**/*.rs\n/exit\n" | \
  OPENAI_API_KEY=$GEMINI_KEY claw --provider gemini --model gemini-2.5-flash
```

### 場景2：自動任務執行
```bash
# 分解並執行大型任務（--yes 喺 task 之前）
printf "/batch --yes 為所有 public function 加 unit tests\n/exit\n" | claw ...
```

### 場景3：知識捕捉
```bash
# 對話後蒸餾重要資訊
printf "我哋決定用 PostgreSQL\n/dream\n/exit\n" | claw ...
# → 自動寫入 .claw/CLAUDE.md，下次啟動自動載入
```

### 場景4：生成 Skill
```bash
# 從對話生成可重用 SKILL.md
printf "{{workflow_conversation}}\n/skillify .claw/skills/my-workflow.md\n/exit\n" | claw ...
```

### 場景5：Coordinator 模式
```bash
# 複雜任務用 KAIROS
printf "/kairos 分析整個 codebase 嘅安全問題\n/exit\n" | claw ...
```

---

## Session 路徑

```
{working_directory}/
└── .claw/
    ├── sessions/          # 對話歷史 (.jsonl)
    ├── CLAUDE.md          # 持久記憶（自動載入）
    ├── instructions.md    # 額外指令（自動載入）
    ├── buddy.json         # Buddy companion 資料
    ├── skills/            # 可用 skills
    └── plugins/           # 插件
```
