<p align="center">
  <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" alt="project-logo">
</p>
<p align="center">
    <h1 align="center">MATRIXMAIL</h1>
</p>
<p align="center">
    <em>Bridge between email and Matrix — forward, respond, automate.</em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/atareao/matrixmail?style=flat&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/actions/workflow/status/atareao/matrixmail/ci.yml?style=flat&logo=githubactions&logoColor=white&color=0080ff" alt="ci">
	<img src="https://img.shields.io/github/last-commit/atareao/matrixmail?style=flat&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/badge/Rust-1.85%2B-dea584?style=flat&logo=rust&logoColor=white" alt="rust">
	<img src="https://img.shields.io/github/languages/top/atareao/matrixmail?style=flat&color=0080ff" alt="top-language">
	<img src="https://img.shields.io/github/languages/count/atareao/matrixmail?style=flat&color=0080ff" alt="language-count">
	<img src="https://img.shields.io/badge/tests-37%20passing-2ea44f?style=flat&logo=rust" alt="tests">
</p>

---

## Overview

**Matrixmail** is a Rust daemon that bridges IMAP email inboxes with [Matrix](https://matrix.org/) chat rooms. It polls an email account for unread messages, forwards them into a configured Matrix room, and lets you interact with them through an OpenAI-compatible LLM backend.

---

## Features

- **Email → Matrix forwarding** — unread emails are automatically posted into a Matrix room
- **LLM-powered chat** — interact with OpenAI-compatible APIs from Matrix rooms
- **Multi-prompt support** — define multiple system prompts and address each one by its first letter (`!h <question>`, etc.)
- **Weather queries** — built-in `!t` command fetches weather for Silla, Spain
- **Docker support** — multi-arch images (arm/v7, arm64, amd64)
- **Idempotent sync** — tracks Matrix sync state via `since` token, persisted to config

---

## Repository Structure

```
matrixmail/
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
├── docker-compose.yml
├── config.sample.yml
├── config.yml              # (generated at runtime)
└── src/
    ├── main.rs             # Entrypoint, Matrix sync loop, process_response
    └── models/
        ├── mod.rs          # Re-exports
        ├── config.rs       # YAML config read/write
        ├── mail.rs         # Email header/body parsing
        ├── imap.rs         # IMAP connection & fetch
        ├── matrix.rs       # Matrix client API (sync, post)
        ├── bot.rs          # Command dispatcher (!?, !h, !t, !<letter>)
        ├── openai.rs       # OpenAI-compatible API client
        └── error.rs        # Custom error types
```

---

## Configuration

Copy `config.sample.yml` to `config.yml` and fill in your credentials:

```yaml
pull_time: 300
imap_server:
  host: mail.example.com
  port: 993
  user: your@email.com
  password: "your-password"
matrix_client:
  protocol: https
  server: matrix.example.com
  token: "syt_..."
  email_room: "!email"
  chat_room: "!chat"
  sender: yourbot
  timeout: 30000
openai_client:
  protocol: https
  server: api.openai.com
  api_key: "sk-..."
  model: gpt-4
  temperature: 0.7
  prompts:
    historia:
      prompt: "Eres un historiador experto"
      messages: []
```

---

## Commands (Matrix chat)

| Command | Description |
|---|---|
| `!?` | Show available commands |
| `!c <prompt>` | Clear message history for a prompt |
| `!h` | Current Unix epoch time |
| `!h <question>` | Ask the `historia` prompt |
| `!t` | Weather in Silla, Spain |
| `!<letter> <question>` | Ask a prompt by its first letter |

---

## Getting Started

### Prerequisites

- Rust 1.85+ (edition 2021)
- An IMAP-enabled email account
- A Matrix account with access token
- (Optional) An OpenAI-compatible API endpoint

### Installation

```console
$ git clone https://github.com/atareao/matrixmail.git
$ cd matrixmail
$ cargo build --release
```

### Configuration

```console
$ cp config.sample.yml config.yml
$ # edit config.yml with your credentials
```

### Usage

```console
$ cargo run --release
```

### Docker

```console
$ docker compose up -d
```

Or build and push multi-arch images:

```console
$ just buildx
```

### Running Tests

```console
$ cargo test
```

37 unit tests covering configuration parsing, email parsing, Matrix response processing, OpenAI client logic, bot commands, and error handling.

---

## Architecture

```
┌──────────┐   poll    ┌───────────┐   forward   ┌────────────┐
│  IMAP    │ ────────> │           │ ──────────> │  Matrix    │
│  Server  │           │ matrixmail│             │  Room      │
└──────────┘           │           │             └────────────┘
                       │  daemon   │
┌──────────┐  respond  │           │  command    ┌────────────┐
│  OpenAI  │ <──────── │           │ <────────── │  Matrix    │
│  API     │           └───────────┘             │  Room      │
└──────────┘                                     └────────────┘
```

---

## License

MIT — see [LICENSE](./LICENSE).

Copyright (c) 2023 Lorenzo Carbonell
