# Agent rules

Always reply in English only.

## Project memory (saved automatically from earlier sessions)
Background only: if it conflicts with the code you actually see, trust the code and say so.

### Project
Horizon Mechanical Works (HMW) is a Rust-based mixin for the Horizon framework that implements mechanical automation systems including power generation, consumption, and machine management.

### Architecture and key files
- `mixins/mechanical_works/runtime.rs` - Main runtime entry point for HMW mixin
- `mixins/mechanical_works/mechanical/power/types.rs` - Foundation power system types (Watts, PowerSupply, PowerDemand, etc.)
- `mixins/mechanical_works/mechanical/power/generator.rs` - Generator subsystem with state management
- `mixins/mechanical_works/mechanical/power/consumer.rs` - Consumer subsystem with allocation tracking
- `mixins/mechanical_works/mechanical/power/grid.rs` - Power grid for managing generators and consumers
- `mixins/mechanical_works/mechanical/machine/types.rs` - Machine system foundational types (MachineId, MachineTypeId, etc.)
- `mixins/mechanical_works/mechanical/machine/state.rs` - Machine state management system
- `Cargo.toml` - Project manifest with proper runtime path references
- `horizon-mixin.toml` - Mixin configuration file

### Conventions and decisions
- All power values represented in consistent watts (W) units
- Modular architecture with separate physical files for each responsibility
- Lightweight module entry points that delegate implementation to submodules
- Strong typing with proper error handling for invalid states/values
- Deterministic power allocation algorithms that respect consumer requirements
- State transitions validated through explicit rules with structured errors
- Unit tests included in each module file for comprehensive coverage

### Commands
- `cargo check --all-targets --manifest-path Cargo.toml` - Verify compilation
- `cargo fmt --all --check --manifest-path Cargo.toml` - Check code formatting
- `cargo build --manifest-path Cargo.toml` - Build project
- `cargo test --manifest-path Cargo.toml` - Run tests
- `cargo test --manifest-path Cargo.toml --lib mechanical::power::types` - Test specific module

### Pitfalls to avoid
- Don't duplicate power type definitions; reuse existing types from power/types.rs
- Ensure all modules are properly exported through parent mod.rs files
- Validate state transitions explicitly to prevent invalid machine states
- Handle overflow conditions safely using saturating arithmetic
- Maintain strict separation between power system logic and Horizon framework integration

### Status and next steps
Project structure successfully established with complete runtime integration. Core power system types, generators, consumers, and grid implemented with comprehensive unit tests. Machine system foundational types and state management completed. Next steps involve implementing Horizon hooks for integration and connecting the mechanical systems to actual game mechanics.

## Workflow for every coding task (Rust / Cargo project)
1. **Explore first.** Use glob and grep to find every file and line range the change touches. Read only what you need.
2. **Plan.** Write a short list of the files you will change and why.
3. **Edit all affected files** (several files per task is expected) before checking anything.
4. **Verify.** Run `cargo check --all-targets`. If tests exist, also run `cargo test`.
5. **Fix loop.** If there are errors: read the FIRST error completely, fix its root cause, run the check again.
   Repeat up to 8 times. If the same error survives 2 attempts, re-read the surrounding code and the full message
   instead of guessing. After 8 rounds stop and report what is still failing.
6. **Finish.** Run `cargo fmt` and `cargo test`, then summarise: files changed, commands run, final result.

## Large files (2000+ lines are normal here)
- Never rewrite or re-output a whole existing file. Find the location with grep, read just that range (offset/limit),
  then change it with `edit` using an exact, unique `oldString` and the full `newString`.
- A 2000-line file is roughly 28,000 tokens. Do not read the same file again and again; keep notes of line numbers.
- Creating a NEW file longer than about 400 lines: write it in sections. Write the first section ending with the line
  `// @@CONTINUE@@`, then use `edit` to replace that marker with the next section followed by the marker again, and so on.
  The last section must remove the marker.

## Completeness (hard rules - never write unfinished code)
- Every function must be fully implemented. Never write `todo!()`, `unimplemented!()`, `// ...`, `// rest of the code`,
  "same as above", "etc." or any other placeholder or elision, and never shorten a list or a match by "and so on".
- If a task needs many similar items (for example 100 stages), write every one of them out.
- After every write or edit run `cargo check`. The errors "unexpected end of file", "unclosed delimiter" or
  "this file contains an unclosed delimiter" mean your previous output was CUT OFF: read the end of the file and finish it.
- Before you report the task as done, use the grep tool to search the changed files for
  `todo!`, `unimplemented!`, `@@CONTINUE@@` and `// ...`. The result must be empty.

## Safety
- Never delete files or directories, and never run `git push`, `git reset --hard`, `git clean` or `sudo`.
- Do not install global tools or add dependencies without saying why.
- Do not change a test just to make it pass; if a test is wrong, say so explicitly.
- Do not edit `Cargo.lock` by hand.
- Prefer small steps that compile over large rewrites.

## Rules learned from verified compile/test failures
- (none yet)
