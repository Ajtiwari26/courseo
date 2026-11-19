# Etta Harness Issue Tracker

This document tracks all runtime anomalies, governor glitches, and tool execution issues encountered when delegating autonomous tasks to the Etta harness.

---

### [ETH-001] Automated Acceptance Gate Infinite Self-Repair Loop on Non-Rust/Frontend Tasks
- **Date & Version:** 2026-11-20 | Etta v2.31.74
- **Model Involved:** Gemini 4 Neon (`--model neon` / `e6fa609c3fa255c0`)
- **Severity:** High (Causes autonomous task to report failure after hitting 10 self-repair cycles even when deliverables are 100% written and valid)
- **Observed Terminal Symptoms & Logs:**
  ```text
  ● file::write  Wrote web/courseo.css
  ● file::write  Wrote web/index.html
  ● file::write  Wrote web/courses.html
  ● file::write  Wrote web/course_detail.html
  ● file::write  Wrote web/classroom.html
  ● file::write  Wrote web/weborient_manifest.json
  ● file::write  Wrote web/weborient_agent.js
  → Quality Gate: Checking stage Lint...
  → Verification: Validating acceptance gate criteria...
  → Verification: Edge Case validation...
  → Resolving Compiler Diagnostics (Attempt #1) Edge case verification incomplete — tests must be executed
  ...
  → Resolving Compiler Diagnostics (Attempt #10) Compiler gate error — repairing build/test failure
  Autonomous task failed: Exceeded maximum self-repair attempts (10)
  ```
- **Root Cause Diagnosis:**
  When Etta runs in a workspace with a root `Cargo.toml`, the governor's verification phase enforces an invariant that test execution tool calls must occur before accepting turn completion. For pure frontend/HTML/CSS/JS tasks where the model only writes web files, the model does not issue a `cargo test` command. Etta interprets the absence of explicit test execution as an incomplete edge-case verification gate failure, re-prompting the model with compiler repair instructions in a loop until reaching the 10-attempt cap.
- **Current Status & Fix:**
  - **Deliverable Status:** Deliverables were successfully written to disk on turn 1 (`web/courseo.css`, `web/index.html`, `web/courses.html`, `web/course_detail.html`, `web/classroom.html`, `web/weborient_manifest.json`, `web/weborient_agent.js`).
  - **Resolution / Workaround:** When invoking Etta on frontend or hybrid tasks in Rust repositories, explicitly instruct the model in the goal prompt to execute `cargo test --all-targets` and an HTML/JS syntax check so that Etta's verification gate satisfies its test execution requirement. In Etta core, the governor should inspect file change scopes and conditionally skip Rust-specific test invariants for frontend-only file writes.
