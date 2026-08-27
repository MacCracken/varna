# Contributing to Varna

Thank you for your interest in contributing to Varna.

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run `sh scripts/check.sh` to validate
5. Open a pull request

## Prerequisites

- Cyrius toolchain (`cyrius`), pinned in `cyrius.cyml` (`cyrius = "6.5.35"`).
  Install via `curl -sSf https://raw.githubusercontent.com/MacCracken/cyrius/main/scripts/install.sh | sh`
- `cyrius deps` to resolve the stdlib snapshot (+ `bote` for `-D MCP`) into `lib/`

## Cyrius Commands

| Command | Description |
|---------|-------------|
| `cyrius deps` | Resolve stdlib + git deps into `lib/` |
| `cyrius deps --verify` | Check `lib/` against the committed `cyrius.lock` |
| `cyrius build src/main.cyr build/varna` | Build the engine + demo entry (`-D LOGGING -D MCP -D DAIMON -D HOOSH` for optional surfaces) |
| `cyrius distlib` | Bundle `src/` → `dist/varna.cyr` for consumers |
| `cyrius tests` | Run `tests/*.tcyr` (recursive) |
| `cyrius bench tests/varna.bcyr` | Run benchmarks (`tests/*.bcyr`) |
| `cyrius fmt <file> --check` | Formatting gate |
| `cyrius lint <file>` | Static analysis |
| `cyrius vet src/main.cyr` | Audit include dependencies |
| `cyrius deny src/main.cyr` | Enforce project policies |
| `cyrius doc --check src/main.cyr` | Doc currency gate |
| `sh scripts/check.sh` | Local gate (deps + lock verify + fmt + lint + build + full build + tests) |

`./scripts/bench-history.sh` snapshots `cyrius bench` into the CSV trend history.

## Adding a Module

1. Create `src/module_name.cyr` with a module doc comment
2. Add it to the `[lib]` (and `[lib.core]` if non-optional) module list in `cyrius.cyml`,
   and `include` it in `src/main.cyr` — **order matters** (Cyrius is single-pass; a module
   must be included after everything it references)
3. If the module needs a new stdlib dep, add it to `[deps].stdlib`
4. Add tests in `tests/` (`*.tcyr`)
5. Update the README module table

Gate optional surfaces behind a `-D` define (`#ifdef NAME` … `#endif`), not a Cargo feature.

## Adding a Language

1. Add a constructor function in the relevant module (e.g., `phoneme_arabic()`)
2. Include the full phoneme inventory with IPA transcription
3. Add grammar profile and script metadata
4. Include at least 20 Swadesh list entries in the lexicon
5. Add tests verifying inventory counts and key features
6. Add a `.bcyr` benchmark for inventory construction

## Code Style

- `cyrius fmt` — mandatory
- `cyrius lint` — zero warnings (`#skip-lint` exempts a line when justified)
- Doc comments on all public items
- Tagged enums for public sum types (additive variants)
- Avoid `@unsafe` blocks
- No raw `print` — log via `lib/log.cyr` / `lib/sakshi.cyr` (under `-D LOGGING`)
- No implicit aborts — check sentinel/`Result` returns from every fallible call

## Testing

- Tests live in `tests/*.tcyr` (run by `cyrius tests`)
- Define-gated tests with `#ifdef NAME` for optional surfaces
- Target: 80%+ line coverage (`cyrius coverage`)

## Commits

- Use conventional-style messages
- One logical change per commit

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
