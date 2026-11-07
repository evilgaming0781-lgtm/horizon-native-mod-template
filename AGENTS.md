# Agent rules

Always reply in English only.

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
