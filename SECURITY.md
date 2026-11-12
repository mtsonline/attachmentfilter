# Security Policy

AttachmentFilter is a security product; we treat vulnerabilities in it accordingly.

## Reporting a vulnerability

Please **do not open a public issue**. Use GitHub's private vulnerability reporting:
*Security → Advisories → Report a vulnerability* on this repository.

Include affected component and version, a description, and steps to reproduce. We aim to acknowledge reports within 3 working days.

## Scope of particular interest

- Filter bypass: a message whose attachment is seen differently by the filter and by common mail clients
- Sandbox escape from an analyzer
- Cross-tenant data access
- Access to quarantined content without the required permission
- Tampering with the audit log that verification does not detect
- Any path by which an accepted message can be lost

## Supported versions

There is no stable release yet. Once there is, the latest minor release receives security fixes.
