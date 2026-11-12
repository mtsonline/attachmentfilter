# ADR 0003: What belongs in the core, what is a plugin

Status: accepted (2026-11-12)

## Rule

Something is built into the core only if it **cannot work as a plugin** – because it enforces a security invariant, or because plugins depend on it. Everything else is a plugin.

## Core

- Milter and SMTP decision logic (ADR 0001)
- MIME parsing and attachment extraction
- Sandbox runner (process isolation, resource limits)
- Policy engine and its DSL
- Quarantine state machine and the store-before-delete ordering
- Envelope encryption (the *key provider* is a plugin, the encryption is not)
- Tenant isolation and RBAC enforcement
- Audit log with hash chain
- Plugin host: discovery, signature verification, lifecycle, capability checks

## Plugins

| Kind | Examples |
|---|---|
| Authentication provider | local accounts, OIDC, LDAP |
| KMS / key provider | file/TPM, HashiCorp Vault, AWS KMS |
| Storage driver | local filesystem, S3-compatible |
| Analyzer | file type, Office macros, PDF, archives, ClamAV, YARA |
| Notification channel | SMTP, Slack, Microsoft Teams |
| Event sink / webhook | generic webhook, syslog, Elastic, Splunk |
| Ticketing, external sandbox | Jira, ServiceNow, CAPEv2 |

Reference plugins that every installation needs (local auth, file key provider, local filesystem storage, the core analyzers) ship in this repository and use exactly the same contract as third-party plugins.

## Contract

- Plugins are **separate processes** speaking gRPC over a Unix socket. No in-process plugins, no dynamically loaded code in the backend.
- Each plugin kind has a versioned protobuf service under `proto/`.
- A plugin declares its capabilities in a manifest; the host grants nothing beyond them.
- Plugin artifacts are signed; unsigned plugins need an explicit operator override.
- A failing plugin must not take the core down: calls have deadlines, and each kind defines its failure behaviour (an analyzer failure means "no verdict", which holds the message – see ADR 0001).
- Frontend extension is limited to declarative contributions (labels, links, settings forms rendered by the core UI). Plugins do not inject JavaScript into the admin UI.
