# WordCraft — deterministic test execution

**Fork:** `Rwanbt/wordcraft`  
**Starting reference:** `main@b30cda15352093710e9346ca1592618403789081`  
**Quality branch:** `test/artcraft-quality-foundation-20261012`

## Environment and evidence boundary

The GitHub repository was audited by the connected GitHub API. The available local shell has no Rust compiler and cannot reach github.com for cloning. Accordingly, local cargo tests and coverage were **not** executed during the initial implementation. GitHub Actions has been configured for actual builds and is the authoritative evidence source.

Do not report success until a workflow run against the current branch head has finished with green checks. For a new push always compare the run's `head_sha` to the branch's current commit SHA.

## First PR validation

`.github/workflows/fork-quality.yml` runs:
1. Rustfmt on Linux.
2. Native `safe_save` and `malformed` integration suites independently on Linux, Windows and macOS.
3. Clippy with warnings denied for the geometry, DOCX and engine packages.
4. A **geom-only** LLVM coverage artifact on Linux.

The pre-existing workflows cover packaging, FreeBSD, release and Windows ARM64; they do not replace a general-purpose workspace/unit/integration test gate.

## Required expansion

- Entire workspace test collection: `cargo xtask stats --exact` and `cargo test --locked --workspace`.
- Full workspace Clippy, assets and layering checks: `cargo xtask ci` (plus dependencies for wasm).
- Full per-crate measured coverage, with explicit exclusions only if justified.
- Native GUI smoke, browser Playwright or equivalent, platform packaging and installation tests.
- Deterministic fixtures for document corruption, concurrency, cancellations and error paths.
- Archive raw CI job URLs and downloaded LCOV artifacts for reproducible cross-commit comparisons.

No generated or hypothetical metric counts as a measured baseline.

## First observed CI evidence

- `38268353195` (SHA `e5e22044f4c8866a96ba617d99b4557ba84f74a8`): formatting **success**; engine unit suite **375/375 success**; core integrations **success** on Linux (8 save + 10 malformed DOCX), macOS (8+10) and Windows (6+10); Clippy failed on a DOCBIN irrefutable let chain.
- `38269866056` (SHA `483918eeb3975004079dbf82e5fe87d4cff1bc83`): formatting, targeted integrations on Linux/macOS/Windows, and geometry LLVM coverage **success**; Clippy failed on two `find_pattern` simplification warnings plus test-only `drop` of a non-`Drop` closure.
- `38271257922` (SHA `ee059393a1f67b9dcaeb2b60430af4da3cbbbcdf`): geometry coverage reports **267 / 268 lines (99.63%)** and **43 / 43 functions (100%)**; formatting failed on two rustfmt arrangements in new tests. These changes are corrected in the subsequent commit; latest exact-SHA CI must be checked separately.

Do not interpret a passing individual job as an entirely green workflow. The package-specific instrumentation and three targeted integration suites do not prove full workspace stability.
