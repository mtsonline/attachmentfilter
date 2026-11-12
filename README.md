# AttachmentFilter

Secure attachment filter with encrypted quarantine for Postfix on Debian.

> **Status: early development (M0 – bootstrapping).** Nothing here is ready for production mail yet.

## Goals

- **No message loss.** Postfix's queue stays the durable buffer until a message is either delivered or provably stored in quarantine.
- **Fail closed.** If a message cannot be analysed in time it is held, not delivered. If the filter itself is unavailable, senders get a temporary `4xx` and retry.
- **Confidential by default.** Quarantined mail is envelope-encrypted per message; access is role-based and fully audited.
- **Small core, everything else a plugin.** Authentication providers, storage and KMS drivers, analyzers, notifications, webhooks and third-party integrations are out-of-process plugins behind stable gRPC contracts.

## Components

| Path | Language | Purpose |
|---|---|---|
| `mx/` | Rust | Milter, MX agent and sandbox runner – everything that touches untrusted mail |
| `backend/` | Go | API, policy engine, audit log, plugin host |
| `cli/` | Go | `attachmentfilterctl`, local admin tool |
| `frontend/` | TypeScript | Static single-page admin UI, no heavy framework |
| `analyzers/` | mixed | Sandboxed analyzers and their SDK |
| `deploy/` | – | Debian packages (MX node) and Docker Compose (backend, frontend, database) |
| `docs/` | – | Architecture decisions, operations, compliance |

Start with [`docs/architecture/`](docs/architecture/) for the design decisions and [`docs/roadmap.md`](docs/roadmap.md) for the plan.

## Building

```sh
(cd mx && cargo test)            # Rust: milter, agent, core
(cd backend && go test ./...)    # Go: backend
(cd frontend/app && npm ci && npm run build)
```

## License

[GNU Affero General Public License v3.0 or later](LICENSE).
