# Contributing

Thanks for your interest. The project is at an early stage; please open an issue before starting larger work.

## Ground rules

- Read the decisions in [`docs/architecture/`](docs/architecture/) first. Changes that contradict an ADR need a new ADR.
- New functionality is a plugin unless it cannot be one (ADR 0003).
- Code that parses untrusted input needs tests with malformed input, and a fuzz target where practical.
- No new dependency without a reason in the pull request description.
- Security issues go through [SECURITY.md](SECURITY.md), not public issues.

## Before opening a pull request

```sh
(cd mx && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test)
(cd backend && go vet ./... && go test ./...)
(cd frontend/app && npm ci && npm run check && npm run build)
```

## License

By contributing you agree that your contribution is licensed under AGPL-3.0-or-later.
