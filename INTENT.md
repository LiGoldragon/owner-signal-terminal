# INTENT — owner-signal-terminal

*The currently named meta-only wire contract for privileged Persona terminal session lifecycle.
Defines the typed request/reply channel that `persona-harness` uses to create and
retire terminal sessions in the `terminal` component.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this currently named
meta-only `owner-signal-terminal` contract. Workspace-shape intent
stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `terminal/INTENT.md`. Ordinary terminal input,
capture, prompt-pattern, and worker-lifecycle traffic stays in
`signal-terminal/INTENT.md`.

## Why this repo exists

`owner-signal-terminal` is the **meta-only authority surface** for `terminal`.
It carries the requests that create or retire terminal sessions — privileged
because they start or stop child-process state owned by the terminal component.
The owner chain is `persona-orchestrate` → `persona-harness` → `terminal` →
`terminal-cell`: orchestrate orders harness work; the harness knows adapter shape
and orders terminal session lifecycle through this surface; `terminal` owns the
actual component state and session processes. Ordinary callers use
`signal-terminal` and cannot express session lifecycle orders through that
vocabulary.

## The channel shape

The owner channel carries (Layer 1 — contract-local verbs on the wire):

- **Requests:** `CreateSession` (install a named terminal session and start the
  configured child process), `RetireSession` (retire a named session and return
  its terminal exit status when available).
- **Replies:** `SessionCreated` (session accepted; exposes the data-socket path
  for viewers), `SessionRetired`, `OwnerTerminalRequestUnimplemented` (reached the
  meta surface but the runtime path is not built yet).

Shared nouns are imported, not copied: `TerminalName` and `TerminalExitStatus`
from `signal-terminal`, and `signal-persona::WirePath` for session data-socket
paths. The daemon lowers these operations to typed Component Commands (Layer 2,
e.g. `AssertSessionRecord` + `StartChildProcess`) which project to payloadless
Sema labels (Layer 3) for observation.

## Constraints

- Session lifecycle orders live only in the meta contract — ordinary
  `signal-terminal::TerminalRequest` has no `CreateSession` / `RetireSession`
  variant.
- Every meta request is a contract-local verb in verb form; Sema classification
  is daemon-side projection only, never a wire wrapper.
- Wire enums are closed. No `Unknown` escape hatch.
- Shared terminal nouns are imported from `signal-terminal`, not duplicated.
- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses — no Kameo, Tokio, redb, or socket implementation.

## Non-ownership

This crate does not own:

- the `terminal` daemon;
- ordinary terminal input/capture/prompt-gate vocabulary;
- the raw PTY or viewer byte plane (the data socket lives outside the triad);
- runtime permission enforcement;
- sema-engine tables or reducers.

## See also

- `ARCHITECTURE.md` — contract surface, three-layer model, and shared nouns.
- `../terminal/INTENT.md` — daemon-side intent (sessions, child processes, data socket).
- `../signal-terminal/INTENT.md` — ordinary terminal input/capture/prompt contract.
- `../terminal-cell/ARCHITECTURE.md` — the session library the daemon drives.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
