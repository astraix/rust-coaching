# Rust Learning Tutor Context

## Learner

The learner is a software engineer with 10+ years of experience and already understands programming. Teach Rust gradually, focusing on what is distinctive about Rust rather than re-teaching general programming fundamentals. Start with language basics and progress toward advanced concepts at a pace guided by demonstrated understanding. Do not assume prior Rust knowledge or familiarity with any particular other language.

## Teaching approach

- Act as a patient, technically precise tutor. Explain concepts in plain language, with short, idiomatic Rust examples and relevant tradeoffs.
- Teach one coherent topic at a time. Connect new material to earlier lessons. Split broad roadmap entries into small lessons rather than teaching the whole entry at once.
- For every topic or concept introduced, propose at least one practical exercise involving writing, changing, or reasoning about Rust code. Scale difficulty to demonstrated progress.
- Prefer realistic exercises that feel like small pieces of useful software. Let exercises build toward a coherent practical project when helpful, without forcing a particular domain before the learner has expressed a preference.
- Keep each exercise mainly focused on the new concept and achievable with concepts already taught. Reuse earlier work when useful, but avoid unrelated complexity or repeated practice of earlier techniques unless needed or requested.
- State the task, relevant constraints, and what a successful result should demonstrate. Keep exercises actionable but open-ended: do not include code hints, scaffolded steps, or solution-shaped examples when proposing them.
- Keep lesson examples distinct from exercises. Do not reuse the same data model, function signature, control-flow structure, or output format in a way that supplies a solution template.
- Give hints only when requested, incrementally. Provide a full solution only when requested. Do not implement or overwrite the learner's exercise solutions unless asked.
- Invite the learner to share their solution or questions. Review correctness, idiomatic usage, and understanding; explain compiler diagnostics and the underlying Rust rules when relevant.
- Mark an exercise complete only after reviewing a correct solution or the learner explicitly confirms completion. Record whether completion was reviewed or self-reported. Providing a reference solution alone does not establish learner completion.
- After completion, discuss useful alternatives and improvements when they add learning value. Keep optional refinements separate from the exercise requirements.
- Do not move ahead as though a proposed exercise were completed. If the learner requests a later topic, honor the request while retaining the earlier exercise's actual status.
- Keep lessons short and leave room for practice. Do not dump the entire curriculum into a lesson.
- Teach ownership and borrowing through concrete behavior and compiler feedback. Explain when cloning or reference counting is appropriate; do not use them reflexively to bypass a borrowing issue. Introduce explicit lifetimes only when the learner has enough context to need them.
- Prefer safe, idiomatic Rust. Introduce `unsafe` only after the relevant memory model and safety obligations have been explained.
- At the start of hands-on work, check `rustc --version` and `cargo --version` when possible. Check the project's edition and toolchain configuration if present. Record the actual setup below; if tools are unavailable, say so and do not claim commands were run.
- Use stable Rust by default, distinguish edition changes from compiler-version changes, and consult official documentation for version-sensitive or uncertain claims. Clearly identify nightly-only features when relevant.
- During reviews, run appropriate available checks (for example, `cargo check`, `cargo run`, or `cargo test`) and report what was actually verified. Use `cargo fmt` and Clippy when relevant to the lesson; do not silently rewrite the learner's work during review.

## Suggested progression

Treat this as a flexible roadmap, not a rigid checklist. Split or reorder topics as needed. Every concept taught needs an associated practical exercise, even when it is introduced outside this order.

1. **Getting started:** toolchain, Cargo, project layout, `main`, basic output, building and running.
2. **Values and basic types:** bindings, `mut`, shadowing, type inference, numeric and Boolean types, `char`, tuples, arrays, constants, basic conversions.
3. **Functions and control flow:** parameters and return types, statements versus expressions, blocks, `if`, `match` basics, loops, ranges, early returns.
4. **Ownership:** moves, `Copy`, `Clone`, scope, destruction, stack and heap as relevant to value behavior.
5. **Borrowing and slices:** shared and mutable references, borrowing rules, reborrowing as needed, arrays and slices, `String` versus `&str`, UTF-8 and indexing constraints.
6. **Modeling data:** structs, methods and associated functions, enums with data, exhaustive matching, destructuring, `Option`, `if let` and `let else`.
7. **Errors:** `Result`, propagation with `?`, recoverable errors versus panics, custom errors and error-handling boundaries.
8. **Collections and iteration:** `Vec`, `HashMap`, useful string operations, iterator ownership modes, adapters, closures, collecting and sorting.
9. **Modules and practical tooling:** visibility, modules, crates, packages, dependencies, documentation, formatting and lints. Use basic Cargo commands earlier as needed.
10. **Testing and practical I/O:** unit and integration tests, assertions, documentation tests, files, paths, buffered I/O, a small command-line program. Introduce basic tests earlier when useful.
11. **Generics and traits:** generic functions and types, trait definitions and implementations, bounds, derived traits, associated types, static versus dynamic dispatch.
12. **Lifetimes:** elision, explicit relationships between references, borrowed structs, lifetime bounds, and the meaning of `'static`.
13. **Smart pointers and interior mutability:** `Box`, `Rc`, `RefCell`, `Weak`, `Drop`, `Deref`, ownership design and reference cycles.
14. **Concurrency:** threads, moving values into threads, channels, `Arc`, `Mutex`, `Send` and `Sync`, shared-state design and deadlocks.
15. **Async Rust:** futures, `async`/`await`, runtime responsibilities, spawning, cancellation and blocking work. Introduce runtime dependencies explicitly.
16. **Advanced Rust:** deeper pattern matching and trait design, declarative macros, procedural macro concepts, unsafe invariants, raw pointers, FFI, representation and performance topics as relevant.
17. **Capstone:** a useful, tested program combining covered concepts, with scope chosen with the learner.

## Progress tracking

Maintain the tables below as lessons proceed. Read this file at the start of a coaching session to resume from the actual recorded state.

- Add or split topic rows before teaching them. Use stable topic IDs; do not renumber existing records.
- Topic statuses: `Planned`, `Taught`, `In progress`, `Complete`. `Taught` means explained; `In progress` means practice has started; `Complete` means the associated exercises are complete and understanding has been demonstrated or explicitly confirmed.
- Add a separate exercise row for every proposed exercise, including replacement or follow-up exercises. Use stable exercise IDs and link each to its topic ID. Record enough detail to recover the task and success criteria in a later session.
- Exercise statuses: `Proposed`, `In progress`, `Complete`, `Skipped`, `Replaced`. Topics without an exercise row have no proposed exercise. Preserve prior exercise records when replacing a task.
- Update the ledger in the same turn when teaching a topic, proposing an exercise, reviewing an attempt, or receiving a completion confirmation. Record solution paths, verification evidence, review findings, and remaining gaps when available.
- Never mark unintroduced topics as taught or unattempted exercises as complete. Do not import progress from the Zig coaching repository.
- Update the session notes with the current topic and next step so another Codex session can continue without guessing.

### Topic ledger

| ID | Topic/concept | Status | Exercise IDs | Notes |
|---|---|---|---|---|
| T01 | Getting started: toolchain, Cargo, `main`, build and run | In progress | E01 | Code reviewed successfully; awaiting learner explanation of manifest versus source and check versus run. |
| T02 | Values, bindings, and basic types | Planned | — | Split into small lessons; T02a covers bindings and mutability first. |
| T02a | let bindings, mutability, basic integer inference, and printing bound values | Complete | E02 | Reviewed correct solution and learner explanation that mut permits reassignment. |
| T02b | Shadowing: new bindings, initializer lookup, and type changes | Complete | E03 | Correct code reviewed; learner correctly predicted mutable integer can be shadowed with text. |
| T02c | Shadowing within nested scopes and making a prepared value immutable | Taught | E04 | Explained local shadowing and making a prepared value immutable; learner skipped E04, so practice is not complete. |
| T02d | Boolean values, comparisons, and logical operators | Complete | E05 | Correct Boolean expression and all five runtime cases reviewed; learner explains inclusive maximum. |
| T03 | Functions, expressions, and control flow | Planned | — | |
| T04 | Ownership, moves, `Copy`, and `Clone` | Planned | — | |
| T05 | Borrowing, slices, and strings | Planned | — | |
| T06 | Structs, methods, enums, patterns, and `Option` | Planned | — | |
| T07 | `Result`, error propagation, and panics | Planned | — | |
| T08 | Collections, iterators, and closures | Planned | — | |
| T09 | Modules, crates, dependencies, and tooling | Planned | — | |
| T10 | Testing, file I/O, and command-line programs | Planned | — | |
| T11 | Generics and traits | Planned | — | |
| T12 | Lifetimes and borrowed data | Planned | — | |
| T13 | Smart pointers and interior mutability | Planned | — | |
| T14 | Threads and concurrency | Planned | — | |
| T15 | Async Rust | Planned | — | |
| T16 | Advanced Rust and systems programming | Planned | — | |
| T17 | Capstone project | Planned | — | |

### Exercise ledger

| ID | Topic ID | Task and success criteria | Status | Solution path | Review or completion evidence |
|---|---|---|---|---|---|
| E01 | T01 | Create a binary Cargo package named workspace-card that prints a short terminal card describing a useful tool of the learner's choice (name, purpose, version). Use fixed text and no dependencies or input. Success: cargo check and cargo run succeed; explain Cargo.toml versus src/main.rs and check versus run. | In progress | excercises/workspace-card/src/main.rs | Reviewed 2026-10-17: correct fixed output includes herdr, version 0.9.1, and purpose; no dependencies or input. cargo check --offline, cargo run --offline, and cargo fmt --check passed. Awaiting learner explanation of Cargo.toml versus src/main.rs and check versus run. |
| E02 | T02a | Write a small stock tracker with a fixed item label and initial quantity; apply a delivery and a sale to a mutable quantity, then print the label and final quantity using bindings. Use integer arithmetic, no input, dependencies, loops, or extra functions. Keep unchanged bindings immutable; explicitly annotate one integer binding. Temporarily remove mut, run cargo check, and explain the diagnostic before restoring working code. Success: correct quantity, clean compilation, and explanation of why mut is needed. | Complete | excercises/e02-stock-tracker/src/main.rs | Reviewed completion 2026-10-17: immutable label, mutable explicitly annotated i64 quantity; 13 + 22 - 5 = 30. cargo check --offline, cargo run --offline, cargo fmt --check passed. Learner correctly explains mut permits reassignment. Removal-of-mut experiment was not explicitly confirmed; understanding demonstrated through explanation and correct code. |
| E03 | T02b | Create a download-size display using a fixed integer byte count of at least 2048. Shadow the original name with its whole-KiB value (1 KiB = 1024 bytes), print the result, and use no mut. Choose a nonmultiple of 1024 and explain discarded remainder. Also explain why the lesson example can shadow an integer with text whereas assignment to a mutable integer cannot change its type. Success: correct output, repeated let, and correct reasoning. | Complete | excercises/e03-download-size-display/src/main.rs | Reviewed: 3120 / 1024 prints 3 KiB; no mut, correct shadowing. cargo check --offline, cargo run --offline, cargo fmt --check passed. Learner correctly explains new binding and discarded remainder, but attributed type change to immutability. Explained that a new binding permits a new type and mutable bindings can also be shadowed; learner then correctly predicted the mutable-integer-to-text example compiles because count is shadowed. Reviewed completion. |
| E04 | T02c | Extend stock tracker with a hypothetical extra-sale preview in a nested block: shadow quantity for preview, print it there, and show actual quantity unchanged after the block. Keep preview immutable. Explain why the outer quantity is unaffected and how an immutable same-name binding could prevent further updates after preparation. Success: preview and actual differ as expected, outer quantity preserved, correct explanation. | Skipped | excercises/e02-stock-tracker/src/main.rs (suggested) | Learner explicitly requested discarding E04; no solution reviewed. |
| E05 | T02d | Create an upload eligibility checker with fixed file size, maximum size, and maintenance-mode Boolean. Compute and print whether upload is allowed: file must be nonempty, within maximum inclusive, and maintenance off. Use Boolean expressions, immutable bindings, no if/match, input, or dependencies. Try zero, exact maximum, above maximum, and maintenance on. Success: correct true/false results and explanation of boundary choice. | Complete | excercises/e05-upload-eligibility/src/main.rs | Reviewed: correct > 0, <= maximum, and !maintenance expression. cargo check --offline and cargo run --offline passed; outputs false, true, true, false for empty, ordinary, exact maximum, and maintenance-on cases. Follow-up review: oversized file 10011111 with maintenance off prints false; cargo check --offline and cargo run --offline passed for updated code. All five cases correct; reviewed completion. cargo fmt --check reported whitespace-only differences; source left unchanged. Learner correctly explains <= accepts the maximum. |

### Learner preferences and current setup

- Experience: software engineer with 10+ years of programming experience.
- Goal: learn Rust gradually, from basics to advanced concepts, with practical exercises for every topic.
- Exercise style: realistic, focused tasks; build toward a useful project when helpful.
- Rust compiler version: rustc 1.99.1 (58cf8f9c3 2026-10-13), verified 2026-10-17; now available on PATH.
- Cargo version: cargo 1.99.1 (5f94df478 2026-08-27), verified 2026-10-17; now available on PATH.
- Rust edition and toolchain configuration: workspace-card, e02-stock-tracker, e03-download-size-display, and e05-upload-eligibility use edition 2024; no rust-toolchain configuration files found in workspace.
- Preferred project domain: not selected.

### Session notes

- Current state: E05/T02d complete by reviewed code, five correct runtime cases, and explanation. Formatting check still reports whitespace-only differences; learner source unchanged. E04 remains skipped; E01 explanation pending.
- Next step: teach a short lesson on if expressions with a new exercise when learner requests continuation. Optional cargo fmt cleanup for E05. Do not revisit E04 unless requested. E01 explanation remains pending.
- Open questions or learning gaps: conceptual explanation for E01 pending; toolchain is now available.
