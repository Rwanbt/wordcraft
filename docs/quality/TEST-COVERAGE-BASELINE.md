# WordCraft — instrumented coverage baseline

**Baseline snapshot:** 2026-10-12
**Repository:** `Rwanbt/wordcraft`
**Base branch SHA:** `b30cda15352093710e9346ca1592618403789081`
**Audit method:** GitHub repository tree, source inspection, workflows and Actions run enumeration. Not a locally executed Rust test run.

## Verified inventory

- 20 workspace package manifests under `apps/`, `crates/`, and `xtask/` (plus the workspace root `Cargo.toml`).
- 19 Rust paths under a `tests/` directory in the baseline Git tree; this is a file count, **not a collected test count**.
- Existing tests include document save integrity, DOCX hostile-input parsing, editing and text parity.
- The baseline fork had 4 GitHub Actions workflows and 0 available workflow runs.
- Baseline branch `main` had no protection and no open pull requests.

## Measurements

| Scope | Line coverage | Branch coverage | Functions | Test harness count | Evidence |
| --- | --- | --- | --- | --- | --- |
| Full workspace | Not measured | Not measured | Not measured | Not collected | Requires complete test/coverage execution |
| `wordcraft-geom` | Pending first GitHub Actions artifact | Pending | Pending | Pending | `fork-quality.yml` / `coverage-geom-linux` |
| `wordcraft-engine` | Not measured | Not measured | Not measured | Pending | `core-regressions` safe-save suite |
| `wordcraft-docx` | Not measured | Not measured | Not measured | Pending | `core-regressions` malformed suite |

The geom LCOV output is an **initial, per-package instrumentation check**, not a whole-repository coverage baseline. Do not combine these figures with other crates, infer a workspace percentage, or assert production readiness until comprehensive CI evidence exists.

## Reproducible commands

```sh
cargo fmt --all -- --check
cargo test --locked -p wordcraft-engine --test safe_save
cargo test --locked -p wordcraft-docx --test malformed
cargo clippy --locked -p wordcraft-geom -p wordcraft-docx -p wordcraft-engine --all-targets -- -D warnings
cargo llvm-cov --locked -p wordcraft-geom --lcov --output-path coverage-geom.lcov
# Larger baseline to measure in a later dedicated CI gate:
cargo test --locked --workspace
cargo xtask stats --exact
```

The first five commands are configured for GitHub Actions, **not marked passed** at documentation creation. Do not add a threshold without a measured result. No exclusions have been established.

## First measured instrumented report

- **Run:** https://github.com/Rwanbt/wordcraft/actions/runs/38269866056
- **Measured commit:** `483918eeb3975004079dbf82e5fe87d4cff1bc83`
- **Crate:** `wordcraft-geom` only.
- **Lines:** 137 / 195 = **70.26%**
- **Functions:** 27 / 38 = **71.05%**
- **Branches:** unavailable (0 branch records in generated LCOV).
- **Artifact:** `coverage-geom-linux`, GitHub artifact ID `11755364816`.

The measurements preceded the extra geometry edge-case and transform tests. Their coverage must be recalculated by CI on the newer commit. Do not treat this number as workspace coverage.
