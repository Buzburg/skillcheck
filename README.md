# Skillcheck 🛡️

**Give your agent skills a checkup.**

Turn a `SKILL.md` file into a readable report: missing metadata, common risky shell patterns, repeated filler, and a small set of contradictory instructions. Get a score and a Markdown badge you can share.

## Try it

Requires a current stable Rust toolchain:

```sh
git clone https://github.com/Buzburg/skillcheck.git
cd skillcheck
cargo install --path . --locked
skillcheck scan /path/to/SKILL.md
skillcheck fix /path/to/SKILL.md
skillcheck badge /path/to/SKILL.md
```

`fix` prints a preview. Review it, then use `skillcheck fix /path/to/SKILL.md --write` to replace the file. The optimizer removes literal filler phrases; it does not understand intent and may affect quoted examples. Keep your file in version control.

## A report you can act on

| Check | What it looks for |
| --- | --- |
| Structure | YAML frontmatter, name and description |
| Risk patterns | Selected traversal sequences and shell-variable patterns |
| Efficiency | Known filler phrases and estimated document size |
| Consistency | Selected conflicting directives |

The score summarizes these rules. It is **not a security certification**, prompt-injection defense, or proof that a skill is safe. Token counts are word-based estimates; cost output uses a fixed illustrative rate, not your provider's billing.

`scan` exits 1 for a score below 70 or an input error. `badge` produces a static score badge; run it again when the skill changes. Each command accepts one file.

## Verify

```sh
cargo test --locked
cargo build --release --locked
```

Developer beta. See [release verification](RELEASE_VERIFICATION.md). MIT licensed. Built by Buzburg.
