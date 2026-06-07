# skills — owner-signal-terminal

Read this before editing the meta-only terminal contract.

## Required context

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- `signal-terminal/ARCHITECTURE.md`
- `terminal/ARCHITECTURE.md`

## Boundary

This crate owns the privileged meta Signal vocabulary for
`terminal` session lifecycle. It contains no daemon code, no
actors, no sockets, and no storage code.

The ordinary `signal-terminal` crate owns the normal terminal
communication surface: input, resize, capture, prompt patterns, input
gates, worker lifecycle, and read-only session lookup. This crate owns
starting and retiring terminal sessions.

## Invariants

- `CreateSession` and `RetireSession` live here, not in the ordinary
  terminal contract.
- Every request variant declares a contract-local operation head
  through `signal_channel!`.
- Shared terminal nouns such as `TerminalName` and `TerminalExitStatus`
  are imported from `signal-terminal`; do not duplicate them.
- Runtime interpretation stays in `terminal`.
