# SPEC: Explicit Recipient Addressing

**Status:** Agreed requirements; interface details pending; not implemented  
**Date:** 2026-10-31  
**Delivery:** Separate follow-up after functional fork consolidation in PR #4

## Purpose

Agents must be able to address mail by a recipient's display name without
first translating that label into a stable agent name. Callers explicitly
choose the identifier type. Existing clients retain existing behavior.

This contract governs Agent Mail's MCP and CLI interfaces. Recording it does
not add recipient resolution to the current release candidate.

## Current behavior

Agent profiles and project directory entries expose both the stable `name`
and optional `display_name`. The directory can therefore be used to discover
the stable address associated with a label. The `whois` tool currently looks
up an agent by stable name. Display labels do not currently resolve message
recipients directly.

## Agreed requirements

### Explicit identifier types

The MCP `to`, `cc`, and `bcc` lists retain their existing string form and gain
recipient objects. Each object contains exactly one selector:

```json
{"to": ["BlueLake"]}
```

```json
{"to": [{"name": "BlueLake"}]}
```

```json
{"to": [{"display_name": "SPX Maintainer"}]}
```

A string keeps its existing stable-address meaning and syntax. A `name`
selector explicitly requests stable-address resolution. A `display_name`
selector requests display-name resolution. Objects with both selectors or
neither selector are invalid.

Agent Mail must never infer the selector type from a string's contents or
retry a failed selector as the other type. A stable name and another agent's
display name may contain identical text: the supplied selector determines
which namespace is searched.

### Resolution and delivery

Display-name resolution is project-scoped. One eligible match resolves to
that agent's stable identity. No match returns a not-found error. Multiple
matches return an ambiguity error identifying the candidate stable addresses;
Agent Mail must not choose a candidate on the caller's behalf.

Resolved recipients remain subject to the existing contact, authorization,
project-boundary, and delivery rules. Display-name addressing grants no new
permission and does not enable broadcast messaging.

Delivery and persisted message references use stable identities. A later
display-name change must not redirect existing messages or their receipts.

### Backward compatibility

Clients using the existing interface must retain the same behavior without
configuration changes or knowledge of this fork. In particular:

- Existing string recipients, including supported project-address syntax,
  retain their meaning in `to`, `cc`, and `bcc`.
- Existing defaults, response shapes, and delivery behavior remain unchanged
  for existing requests.
- Existing CLI flags retain their meanings. Display-name addressing uses
  separate explicit flags; their exact spellings remain to be specified.
- The MCP schema advertises the additional recipient forms. Existing client
  compatibility must be checked against that schema extension as well as
  against server behavior.

## Decisions required before implementation

The following details are not yet settled by this contract:

- Display-name comparison: case sensitivity, whitespace treatment, and
  Unicode normalization. Registration's existing normalization must be
  accounted for explicitly.
- Target-project selection for typed recipients, including whether the
  initial display-name form supports cross-project delivery.
- Exact CLI flags, repeated-flag behavior, and combining stable and display
  selectors in one recipient category.
- Eligibility of retired identities and the information disclosed in
  ambiguity errors, consistent with existing visibility rules.
- Structured error codes and delivery behavior when one recipient fails
  resolution in a message with several recipients. Existing string-only
  request behavior must be preserved.
- Resolution consistency when a label changes concurrently with a send,
  and duplicate-recipient handling when selectors identify the same agent.
- The corresponding contract for explicit recipients in replies and other
  public operations that accept recipient addresses.

Resolve these details in this specification before implementing the feature.

## Acceptance criteria

1. Existing string-only requests and CLI commands retain their behavior,
   including successful delivery, errors, project addressing, and responses.
2. A unique display-name selector delivers to the matched stable identity in
   each of `to`, `cc`, and `bcc`.
3. Duplicate labels produce an ambiguity error with candidate stable
   addresses; an absent label produces a not-found error.
4. When one agent's stable name equals another agent's display name, each
   explicit selector resolves only in its declared namespace.
5. Failure to resolve a selector never falls back to the other namespace.
6. Objects with both selectors or neither selector are rejected.
7. Relabeling after delivery leaves message ownership, receipt ownership,
   and historical delivery destinations unchanged, including after restart.
8. Existing authorization, contact, and project-isolation restrictions remain
   effective for display-name recipients, including BCC confidentiality.
9. Existing-client conformance and the additional MCP and CLI addressing
   cases have passing evidence before the follow-up is considered complete.

These criteria define future verification requirements. No implementation or
passing evidence for the additional recipient forms is claimed here.
