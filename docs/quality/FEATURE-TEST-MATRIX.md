# WordCraft — functional test traceability

**Date:** 2026-10-12. Source inspection is not sufficient to mark tests as executed. States: VERIFIED / PARTIAL / UNVERIFIED / BLOCKED. Until CI has reported on the exact branch SHA, **no new feature is VERIFIED**.

| ID | Component | Expected contract / risk | Existing / added evidence | CI scope | State |
| --- | --- | --- | --- | --- | --- |
| WC-SAVE-001 | `engine/io/safe_save.rs` | Atomic preparation preserves existing destination on failure and does not leak temporary files | Existing unit tests; 3 added edge-case tests for new document rollback, dropping staged write and exact replacement bytes | Rust unit tests transitively via future workspace run; not yet directly configured | UNVERIFIED |
| WC-SAVE-002 | `engine/tests/safe_save.rs` | Failed Save As preserves path, dirty state and existing bytes; encrypted saves round-trip | Existing `safe_save` integration suite | Linux, Windows, macOS `core-regressions` | UNVERIFIED |
| WC-DOCBIN-001 | `docbin/src/fmt.rs` | Binary Word character COLORREF preserves RGB byte order; truncated operand has no effect | Added unit regression alongside a Clippy-corrected, behaviour-preserving parser guard | Linux `linux-quality` DOCBIN unit suite | UNVERIFIED |
| WC-DOCX-001 | `docx/tests/malformed.rs` | Hostile or truncated OOXML/ZIP does not cause crashes or silently invalid output | Existing ZIP/XML/truncation and property-based tests | Linux, Windows, macOS `core-regressions` | UNVERIFIED |
| WC-DOCX-002 | `docx/tests/roundtrip.rs` | Valid OOXML roundtrips retain document semantics | Existing test file, not yet exercised by new CI | Not yet configured | UNVERIFIED |
| WC-DOC-001 | `doc/src/tests.rs` | Model invariants, edits and serialization | Existing test file; tests not harness-counted | Not yet configured | UNVERIFIED |
| WC-ENGINE-001 | `engine/src/tests_typing.rs` | Word-style keyboard editing, undo and list semantics | Existing test module | Not yet configured | UNVERIFIED |
| WC-MCP-001 | `mcp`, `apps/wordcraft-cli` | Commands, root confinement, deterministic error handling | Source exists; CLI smoke and protocol matrix pending | Not yet configured | UNVERIFIED |
| WC-UI-001 | `ui-egui`, `apps/wordcraft` | Open/edit/save/reload, keyboard, dialogs and rendering | Existing screenshot harness noted in AGENTS.md | Native E2E unavailable in this first gate | BLOCKED |
| WC-WEB-001 | `apps/wordcraft-web` | Browser startup, edit, persistence and export | Web target exists; no browser E2E in first gate | Not yet configured | UNVERIFIED |
| WC-GEOM-001 | `geom` | Geometry contract and numerical boundaries | Instrumented geom coverage job added | `linux-quality` | UNVERIFIED |

Next actions: collect real harness counts; inspect missing cases by public command / operation; measure Rust lines/functions/branches package by package; expand integration, UI and browser smoke; add fuzz regression seeds. Source files are only discoverability evidence, never a passing test.
