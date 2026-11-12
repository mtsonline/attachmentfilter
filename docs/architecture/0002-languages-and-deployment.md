# ADR 0002: Languages, deployment and scale

Status: accepted (2026-11-12)

## Languages

| Area | Choice | Why |
|---|---|---|
| Milter, MX agent, MIME parsing, sandbox runner | **Rust** | This code parses hostile input in the SMTP path. Memory safety without a garbage collector, predictable latency, good fuzzing support. |
| Backend API, plugin host, CLI | **Go** | Mature gRPC, PostgreSQL and OIDC ecosystem; simple static binaries; the out-of-process plugin model is well established there. |
| Frontend | **TypeScript**, no heavy framework | Small dependency tree, strict CSP without inline scripts. |
| Analyzers | whatever the tool is written in | Existing tools (ClamAV, oletools, YARA) are wrapped, never linked. They only ever run inside the sandbox. |

A single-language (all-Rust) code base was considered. It would remove one toolchain, but the backend is ordinary CRUD, auth and integration work where Go is quicker to write and review, and the security argument for Rust applies mainly to the mail path.

The milter protocol is implemented in-tree (or via a small audited crate): it is compact, and owning it lets us fuzz it.

## Deployment

- **MX node:** Debian packages (`.deb`) with systemd units and AppArmor profiles for milter, agent and analyzers. They talk to Postfix over a Unix socket.
- **Backend, frontend, PostgreSQL:** container images with a Docker Compose setup. Large installations point the backend at an external PostgreSQL and S3-compatible storage instead of the bundled ones.
- Updates come through the signed APT repository and image tags. There is no self-updating agent inside the product.
- Not planned: Helm charts, Redis (PostgreSQL `SKIP LOCKED` is the job queue), OPA/Rego (one native policy DSL), in-process Go plugins.

## Scale

The design target is "as much mail as the hardware allows, from one box to very large setups":

- Milter and agent are stateless per message; MX nodes scale horizontally behind normal MX records.
- The backend is stateless and can run in several replicas.
- `messages` and `audit_log` are partitioned by time; every table carries `tenant_id` with row-level security from the first migration.
- Message bodies live in object storage, never in PostgreSQL.
- Back-pressure is explicit: bounded queues everywhere, and overload turns into `4xx`, never into dropped mail.
