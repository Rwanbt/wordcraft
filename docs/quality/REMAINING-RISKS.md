# WordCraft — outstanding risks / release blockers

**Audit date:** 2026-10-12  
**Baseline:** `main@b30cda15352093710e9346ca1592618403789081`

## Confirmed gaps

| Priority | Risk | Evidence | Closure condition |
| --- | --- | --- | --- |
| P0 | No reliable fork CI signal at baseline | GitHub Actions query returned zero workflow runs on the fork | Successful runs at exact PR SHA, and enforced required checks after verification |
| P0 | Core editing and persistence cannot yet be declared safe for production | No tests were executed in this session; save tests added but not validated | Core integration and corruption/rollback scenarios pass on supported platforms |
| P0 | No whole-workspace instrumented coverage | No baseline coverage artifacts or percentage available | Per-package and aggregate compatible reports, annotated exclusions |
| P1 | E2E product experience not validated | Native and web end-to-end launch/edit/save/reopen/export not included in first CI | Automated launch + document roundtrip + export checks with artifacts |
| P1 | Multi-platform verification incomplete | Only selective Linux/Windows/macOS suites added; no full desktop installs | Build/package/smoke on each advertised platform; failures documented |
| P1 | Upstream divergence | At baseline upstream main was 56 commits ahead of fork main | Assess changes and merge/rebase only as an independent reviewed operation |
| P1 | No protection on `main` | GitHub branch response lists `protected=false` | Owner enables branch protection and required verified checks |
| P2 | Seeded fuzz/mutation evidence insufficient | Existing hostile DOCX tests inspected, global campaigns not verified | Store reproducible seeds and execute bounded campaigns |

## Current decision

**PARTIALLY VERIFIED (audit and proposed tests only), NOT PRODUCTION READY for the application.** A PR must remain in draft until CI is demonstrably green and broader gates are completed. No automatic main merge, force-push or upstream PR.

## Observed CI issues and mitigations

- Initial CI failed due to Rust 1.90 versus locked dependencies requiring Rust 1.92. Manifest and isolated CI now use Rust 1.92.0.
- One pre-existing `docbin/src/fmt.rs` irrefutable let-chain failed Clippy `-D warnings`. A behavior-preserving fix with a synthetic RGB decoding test was committed; passing Clippy at the newer SHA is still required.
- Initial geometry instrumentation at `483918eeb3975004079dbf82e5fe87d4cff1bc83`: 70.26% lines and 71.05% functions. Geometry tests and numeric overflow validation have been expanded; recalculate coverage after new CI.
- A later strict Clippy pass identified engine `find_pattern.rs` unnecessary negation and a test-only explicit `drop(run)` on a non-`Drop` closure in `ribbon.rs`. Boolean semantics preserved with a complete 8-combination regression; removed redundant explicit drop. Verification at the new SHA pending.

- Expanded workspace CI on 2026-10-12 identified `egui` / `eframe` `0.36.2` and related locked crates requiring Rust 1.95, whereas the earlier narrow-engine checks only required Rust 1.92. The declared workspace MSRV and fork CI toolchains were advanced to 1.95; complete CI at the new SHA is required before this is considered fixed.
