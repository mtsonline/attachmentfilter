# Roadmap

## M0 – Bootstrapping (in progress)

Repository layout, toolchains, CI, security baseline, architecture decisions.

## Phase 1 – Vertical slice

A single-tenant system that filters real mail end to end.

- Milter and MX agent implementing ADR 0001 (inline scan, hold, tempfail)
- MIME parser with fuzzing harness; malformed or ambiguous MIME is treated as suspicious
- Sandbox runner and core analyzers: file type, filename heuristics, Office macros, PDF, archives, ClamAV, YARA
- Plugin contracts (`proto/`) and plugin host
- PostgreSQL schema with `tenant_id` and row-level security from the start
- Encrypted quarantine with store-before-delete, local filesystem storage, file key provider
- Policy DSL with the default profile
- Minimal API and CLI: list, inspect, release
- Audit log with hash chain

## Phase 2 – Operation

- Frontend: dashboard, quarantine explorer, message detail, release
- Auth plugins (local, OIDC) and RBAC, with justification and optional dual approval for content access
- Recipient and admin notifications (EN, DE)
- Debian packages and Postfix installer with rollback; Docker Compose for backend, frontend and database

## Phase 3 – Breadth

- Multi-tenancy in UI and API, per-tenant policies
- Retention and crypto-shredding, legal hold
- S3 storage, Vault key provider
- First integrations: generic webhook, syslog/Elastic, Jira, Slack/Teams
- DR runbooks, benchmarks, chaos tests, signed releases with SBOM

## Dropped from the original concept

OPA/Rego, in-process Go plugins, Redis, Helm charts, QRadar/Sentinel/XSOAR connectors, Azure/GCP KMS, the in-product auto-updater. Any of these can come back later as a plugin.
