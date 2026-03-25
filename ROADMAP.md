# jq-lite Learning Roadmap

This roadmap is designed to help you learn Rust by building a scoped jq-like CLI tool in small, complete steps.

## jq Mental Model (Quick)

- `jq` reads JSON input and applies a filter expression.
- A filter takes one JSON value and can emit one or many outputs.
- Examples:
  - `.` -> identity (return input)
  - `.name` -> object field access
  - `.items[]` -> iterate array elements
  - `map(.id)` -> transform each element
  - `select(.age > 18)` -> keep matching items

---

## Checkpoint 0: CLI + JSON I/O

### Build

- [x] Parse CLI args (`jq-lite '<filter>'`)
- [x] Read JSON from stdin
- [x] Parse JSON into `serde_json::Value`
- [x] Print JSON output (pretty by default)
- [x] Print useful error messages for invalid JSON/filter

### Rust Focus

- [x] Use `Result` + `?` consistently
- [ ] Keep modules separated (`cli`, `error`, `main`)

### Done When

- [x] `echo '{"a":1}' | jq-lite '.'` works end-to-end

---

## Checkpoint 1: Minimal Filter Parser + Evaluator

### Build

- [ ] Support `.`
- [ ] Support field access: `.name`
- [ ] Support chained fields: `.a.b.c`
- [ ] Create AST with `enum`/`struct` nodes
- [ ] Evaluate AST against JSON input

### Suggested Grammar (v0)

- [ ] `Filter := "." ("." IDENT)*`

### Rust Focus

- [ ] Model expressions with `enum Expr`
- [ ] Use pattern matching for evaluation
- [ ] Decide behavior for missing fields (`null` vs error) and document it

### Done When

- [ ] `{"a":{"b":2}}` with `.a.b` returns `2`

---

## Checkpoint 2: Arrays + Iteration

### Build

- [ ] Support indexing: `.items[0]`
- [ ] Support expansion: `.items[]`
- [ ] Print one output per emitted value

### Rust Focus

- [ ] Represent evaluator outputs as stream-like (`Vec<Value>` initially is fine)
- [ ] Keep cloning minimal; understand where ownership forces clones

### Done When

- [ ] `[1,2,3]` with `.[]` prints `1`, `2`, `3` (one per line)
- [ ] Out-of-bounds behavior is defined and tested

---

## Checkpoint 3: Pipes + Constructors

### Build

- [ ] Support pipe: `.users[] | .name`
- [ ] Support array constructor: `[.a, .b]`
- [ ] Support object constructor: `{id: .id, n: .name}`
- [ ] Parse precedence so `|` binds lower than field/index ops

### Rust Focus

- [ ] Compose filters as transformations over emitted values
- [ ] Keep parser modular (`lexer` optional, parser required)

### Done When

- [ ] Can reshape realistic input JSON into new objects/arrays

---

## Checkpoint 4: `select` + Comparisons

### Build

- [ ] Support `select(...)`
- [ ] Add `==`, `!=`, `>`, `<`, `>=`, `<=`
- [ ] Use selection to filter streams (e.g. `.[] | select(.age > 18)`)

### Rust Focus

- [ ] Define comparison semantics across JSON types
- [ ] Return clear errors for invalid comparisons

### Done When

- [ ] Can filter arrays of objects by predicates

---

## Checkpoint 5: Quality Pass

### Build

- [ ] Unit tests for parser
- [ ] Unit tests for evaluator
- [ ] Integration tests for CLI behavior
- [ ] Improve parser errors with location info
- [ ] Add `--compact-output` (`-c`)
- [ ] Optional: add raw string output (`-r`)

### Rust Focus

- [ ] Refactor boundaries (`ast`, `parser`, `eval`, `cli`, `error`)
- [ ] Use tests to drive refactors safely

### Done When

- [ ] Core subset is reliable and easy to extend

---

## Checkpoint 6 (Optional): Stretch Goals

### Build

- [ ] Add built-ins: `map`, `length`, `keys`, `has`
- [ ] Explore limited recursive descent support
- [ ] Explore variables (`as`) or simple functions

### Guardrail

- [ ] Only start this after core checkpoints are stable

---

## Project Guardrails (Keep Scope Healthy)

- [ ] Keep a "Supported jq Subset" section in `README.md`
- [ ] Ship small complete features, not partial big ones
- [ ] For each feature, add:
  - [ ] parser test
  - [ ] evaluator test
  - [ ] CLI/integration test
- [ ] Avoid chasing full jq compatibility early

---

## Suggested Initial Module Layout

- [ ] `src/main.rs`
- [ ] `src/cli.rs`
- [ ] `src/error.rs`
- [ ] `src/ast.rs`
- [ ] `src/parser.rs`
- [ ] `src/eval.rs`
- [ ] `tests/cli.rs`

---

## First Milestone Plan (Week 1)

- [ ] Day 1: Checkpoint 0 complete
- [ ] Day 2-3: Checkpoint 1 complete
- [ ] Day 4-5: Checkpoint 2 complete
- [ ] Day 6: Parser/eval cleanup
- [ ] Day 7: Tests + README subset update

If this feels heavy, cut scope and keep the momentum. A smaller complete jq-lite beats an ambitious unfinished one.
