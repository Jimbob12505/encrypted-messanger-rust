# Rust Based Encrypted Messenger

Terminal-first end-to-end encrypted messenger in Rust.

This repository is organized as a Cargo workspace:

- `bin/cli` — client binary (TUI later)
- `bin/server` — mailbox server
- `core/net` — networking/transport layer
- `core/crypto` — crypto/session abstractions
- `core/proto` — message framing/types
- `tui` — terminal UI components
