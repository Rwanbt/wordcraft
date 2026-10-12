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
