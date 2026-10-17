# skillcheck 🛡️

> **Lighthouse & Security Auditor for AI Agent Skills & Prompts**

[![CI](https://github.com/Buzburg/skillcheck/actions/workflows/ci.yml/badge.svg)](https://github.com/Buzburg/skillcheck/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

`skillcheck` is a blazingly fast Rust linter and security scanner for `SKILL.md` files and AI agent prompt manifests. It audits instruction files across 4 vectors: **Structure**, **Prompt Injection & Shell Safety**, **Token Bloat & Cost Efficiency**, and **Instruction Consistency**—providing actionable diagnostics, automated token pruning (`--fix`), and a verifiable README badge.

```
 ┌─────────────────────────────────────────────────────────────┐
 │ skillcheck v0.1.0 ── Auditing: skills/deploy-k8s/SKILL.md    │
 ├─────────────────────────────────────────────────────────────┤
 │  [PASS] Structure & YAML Frontmatter           100/100      │
 │  [WARN] Token Density & Conciseness             72/100      │
 │         - 840 superfluous conversational tokens detected    │
 │  [PASS] Path & Binary Safety                   100/100      │
 │  [FAIL] Indirect Prompt Injection Resilience   44/100       │
 │         - Unsanitized grep argument allows command escape   │
 ├─────────────────────────────────────────────────────────────┤
 │  OVERALL SKILL SCORE: 79/100 [GRADE: C+]                    │
 │  Markdown Badge: [![SkillScore 79](https://...)]            │
 └─────────────────────────────────────────────────────────────┘
```

---

## ⚡ Why skillcheck?

- **Audit Security in Sub-10ms:** Detects path traversal, unquoted bash variables, and risky parameter expansions.
- **Save Wasted Tokens:** Identifies conversational fluff ("Please ensure that you kindly...") that inflates context windows across thousands of agent steps.
- **Automated `--fix` Compression:** Prunes boilerplate phrases without altering core instructions, reducing token counts by ~30%.
- **The Badge Flywheel:** Generates dynamic GitHub shields badges for skill authors to showcase quality in their READMEs.

---

## 🚀 Quickstart

### Installation

```bash
git clone https://github.com/Buzburg/skillcheck.git
cd skillcheck
cargo install --path . --locked
```

### Usage

```bash
# 1. Audit your SKILL.md
skillcheck scan SKILL.md

# 2. Preview token compression
skillcheck fix SKILL.md

# 3. Apply fixes in place
skillcheck fix SKILL.md --write

# 4. Generate README badge
skillcheck badge SKILL.md
```

---

## 🧪 Testing

```bash
cargo test
```

---

## 📄 License

Licensed under [MIT](LICENSE).
