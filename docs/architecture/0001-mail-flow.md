# ADR 0001: Mail flow – scan inline, hold on timeout, fail closed

Status: accepted (2026-11-12)

## Context

The original concept was "accept and retro-scan": answer `250`, scan after the queue, quarantine afterwards. Once a milter accepts a message Postfix delivers it immediately, so a later scan races delivery and usually loses. Combined with `milter_default_action = accept`, a crashed filter would let all mail through unscanned.

## Decision

1. **Inline scan.** The milter streams the message to the MX agent and waits for a verdict at end-of-message, within a configurable time budget (default 45 s, well below Postfix's `milter_content_timeout`).
2. **Hold on timeout.** If no verdict arrives in time, the milter answers `250` but places the message in Postfix's *hold* queue (milter quarantine action). The agent finishes the scan asynchronously and then either releases the message (`postsuper -H`) or moves it to quarantine.
3. **Tempfail when overloaded or down.** If analyzers have been unavailable longer than `outage.tempfail_after` or the hold backlog exceeds `outage.backlog_threshold`, the milter answers `451 4.7.1` so the sender retries.
4. **Fail closed.** Postfix is configured with `milter_default_action = tempfail`. A dead milter delays mail; it never passes mail unscanned.
5. **Store before delete.** A message leaves the Postfix queue for quarantine only after the encrypted EML is durably written to object storage *and* the metadata transaction is committed. This ordering is the zero-loss guarantee and gets dedicated crash-injection tests.
6. **Reinjection bypasses the filter.** Released mail and notifications are submitted through a dedicated loopback `smtpd` listener without milters, so they are not rescanned or looped.
7. **One tenant per SMTP transaction.** A milter decides per message, not per recipient. Recipients belonging to a second tenant get `452 4.5.3` at `RCPT TO`; the sender delivers them in a separate transaction.
8. **One direction per instance.** An instance filters either inbound or outbound mail (`direction: inbound | outbound`); which mail reaches it is decided by DNS and Postfix routing.

## Consequences

- Nothing is delivered unscanned; the worst case under failure is delay.
- Inline scanning adds latency to the SMTP dialogue; the time budget and worker pools must be tuned and monitored.
- The agent needs local privileges to manage the hold queue.
- "Sanitized delivery" in v1 means removing the attachment and inserting a notice. Modifying a message invalidates its DKIM signature, which matters when mail is forwarded onwards.
