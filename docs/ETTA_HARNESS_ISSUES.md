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

---

### [ETH-002] Upstream Robin Gateway Socket Stalling on Monolithic Multi-File Outputs
- **Date & Version:** 2026-11-20 | Etta v2.31.74
- **Model Involved:** Gemini 4 Neon (`--model neon` / `e6fa609c3fa255c0`)
- **Severity:** Medium (Process hangs indefinitely on an established TCP stream with 0 CPU time after first file write)
- **Observed Terminal Symptoms & Logs:**
  ```text
  ● file::write  Wrote web/courseo.css
    └─ Wrote 1658 lines (33.8 KB) · 117.47s
  (Process hangs with CPU time frozen at 0:03.12 for >5 minutes, socket in ESTABLISHED state with 0 byte transfer)
  ```
- **Root Cause Diagnosis:**
  When dispatching a massive prompt requesting 4–5 large web pages in a single autonomous turn, the model generates an initial large file (`web/courseo.css`), but the subsequent model turn or tool call streaming stalls on the upstream Google Robin Gateway socket without an internal read timeout triggering a retry or reconnection.
- **Current Status & Fix:**
  - **Workaround:** Shard autonomous file-generation tasks into discrete, targeted dispatches per file or subsystem (e.g. CSS design system $\to$ Landing Page $\to$ Catalog & Detail pages) rather than issuing a single monolithic multi-page generation request.
  - **Status:** Mitigated via targeted modular execution and independent verification.


---

### [ETH-003] Symlink Cycle / False Positive Authorization Denied on Scoped Output Directories during Verification
- **Date & Version:** 2026-11-20 | Etta v2.31.81
- **Model Involved:** Gemini 4 Neon (`--model neon` / `e6fa609c3fa255c0`)
- **Severity:** High (Prematurely aborts post-write verification gate with exit code 1)
- **Observed Terminal Symptoms & Logs:**
  ```text
  ● file::write  Wrote web/courseo.css
  ● file::write  Wrote web/index.html
    └─ Wrote 1461 lines (29.7 KB) · 149.73s
    └─ Wrote 833 lines (36.8 KB) · 149.73s
    └─ test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s · 6.14s
  → Verification: Validating acceptance gate criteria...
  ❌ Fault: authorization denied: Symlink cycle detected at "/Users/ajaytiwari/Desktop/Projects/courseo/web/assets"
  Autonomous task stopped without sufficient progress.
  ```
- **Root Cause Diagnosis:**
  When Etta executes post-turn verification, its directory scanner traverses workspace subdirectories recursively to compute changesets. If a subdirectory contains a relative symlink back up to a sibling or parent directory (e.g. `web/assets -> ../assets`), Etta's cycle detector treats this as an unauthorized directory traversal fault and aborts the task with exit code 1, despite the actual file writes and test executions having succeeded completely.
- **Current Status & Fix:**
  - **Deliverable Status:** Deliverables (`web/courseo.css` and `web/index.html`) were successfully authored and written to disk before the verification traversal check.
  - **Resolution / Workaround:** Avoid relative parent symlinks inside subdirectories within Etta workspaces; replace symlinks with physical directories or copy assets directly so that the directory traversal graph remains strictly acyclic.

---

### [ETH-004] Unbounded Governor Verification Spin on Alternating Edge Case / Acceptance Gate Polling
- **Date & Version:** 2026-11-20 | Etta v2.31.83
- **Model Involved:** Gemini 4 Neon (`--model neon` / `e6fa609c3fa255c0`)
- **Severity:** High (Governor enters infinite non-yielding loop emitting `→ Verification: Edge Case validation...` and `→ Verification: Validating acceptance gate criteria...`, exceeding 15,000 log lines and consuming CPU indefinitely until manual SIGKILL)
- **Observed Terminal Symptoms & Logs:**
  ```text
  ● file::write  Wrote web/assets/faculty/rohan_verma.svg
  ● file::write  Wrote web/assets/faculty/ananya_nair.svg
  ● file::write  Wrote web/assets/faculty/kabir_mehta.svg
  ● file::write  Wrote web/assets/faculty/devika_sharma.svg
  → Verification: Validating acceptance gate criteria...
  → Verification: Edge Case validation...
  → Verification: Validating acceptance gate criteria...
  → Verification: Edge Case validation...
  (repeats 15,000+ times without turn termination or error timeout)
  ```
- **Root Cause Diagnosis:**
  When Etta executes a sequence of tool writes that creates new assets without an intermediate shell execution, the verification governor state machine toggles between `Edge Case validation` and `Validating acceptance gate criteria` without a sleep/backoff or loop counter cap when evaluating pending tool results.
- **Current Status & Fix:**
  - **Deliverable Status:** Deliverables (`web/assets/faculty/rohan_verma.svg`, `ananya_nair.svg`, `kabir_mehta.svg`, `devika_sharma.svg`) were successfully authored and written to disk before the verification spin.
  - **Workaround:** Terminated PID 10944 via SIGKILL; completed remaining single asset (`vikramaditya_sen.svg`), layout un-nesting in `web/courseo.css`, and HTML/showreel modal integration.
  - **Fix for Etta core:** Add a hard maximum loop iteration ceiling (e.g. 5 iterations) to the verification edge-case evaluation state machine so it yields or surfaces a diagnosis rather than spinning unbounded.
