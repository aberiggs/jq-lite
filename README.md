# jq-lite

A small jq-inspired CLI tool built in Rust.

## Planned jq Subset

- [x] Identity: `.`
- [x] Field access: `.name`, `.a.b`
- [x] Array indexing: `.[0]`, `.items[2]`
- [x] Array iteration: `.[]`, `.items[]`
- [ ] Pipe: `.users[] | .name`
- [ ] Constructors: `[.a, .b]`, `{id: .id, name: .name}`
- [ ] `select(...)` with basic comparisons (`==`, `!=`, `>`, `<`, `>=`, `<=`)
- [ ] CLI polish (`-c`, optional `-r`) and improved parser errors

## Current Examples

### Example 1: Nested field access

Input command:

```bash
echo '{"a":{"b":2}}' | cargo run -- '.a.b'
```

Expected output:

```json
2
```

---

### Example 2: Array iteration

Input command:

```bash
echo '{"users":[{"name":"Ada"},{"name":"Linus"}]}' | cargo run -- '.users[].name'
```

Expected output:

```json
"Ada"
"Linus"
```

---

### Example 3: Array index access

Input command:

```bash
echo '{"items":[10,20,30]}' | cargo run -- '.items[1]'
```

Expected output:

```json
20
```
