# SPEC: Explicit Recipient Addressing

**Status:** Requirements and interface agreed
**Dates:** requirements 2026-10-31, interface decisions 2026-11-15
**Delivery:** its own pull request, after the fork has moved to current upstream

## Purpose

Agents must be able to address mail by a recipient's display name without first translating that label into a stable agent name. Callers explicitly choose the identifier type. Existing clients retain existing behavior.

This contract governs Agent Mail's MCP and CLI interfaces.

## Behavior before this change

- Agent profiles and directory entries expose the stable `name` and the optional `display_name`.
- A display name is stored trimmed and otherwise as typed: at most 128 characters, no control characters. Two agents may carry the same display name.
- `send_message` and `reply_message` take lists of strings in `to`, `cc` and `bcc`. An object in a list is rejected with `INVALID_ARGUMENT`.
- A recipient string is a plain name, `Name@project`, `project:<project>#Name`, or `project:<slug>` for a shared project mailbox. Stable names match without regard to case.
- One unresolvable recipient fails the whole send before anything is written. By default an unknown plain local name is registered on the spot.
- Nothing resolves a recipient from a display name.

## Requirements

### Explicit identifier types

The MCP `to`, `cc` and `bcc` lists retain their string form and gain recipient objects. Each object contains exactly one selector:

```json
{ "to": ["BlueLake"] }
```

```json
{ "to": [{ "name": "BlueLake" }] }
```

```json
{ "to": [{ "display_name": "SPX Maintainer" }] }
```

A string keeps its existing stable-address meaning and syntax. A `name` selector explicitly requests stable-address resolution. A `display_name` selector requests display-name resolution. An object with both selectors, with neither, or with any other field is invalid.

The `name` selector accepts exactly what a string accepts, including the project-address forms and registration on the spot. It differs from a string only in being explicit.

An object with a `project` field is invalid, like an object with any other additional field. That keeps the name free for a later cross-project form.

Agent Mail must never infer the selector type from a string's contents or retry a failed selector as the other type. A stable name and another agent's display name may contain identical text: the supplied selector determines which namespace is searched.

### Comparison

A `display_name` selector is trimmed and then compared with stored display names without regard to case. Inner whitespace and Unicode form must match the stored label.

"Without regard to case" covers all letters, not only ASCII: `MÜLLER` finds `Müller`. Stable names keep their ASCII-only comparison.

### Project scope

A `display_name` selector searches the sender's own project only. Mail to another project keeps using stable address strings.

### Candidates

Only active agents are candidates. A label shared by a retired agent and an active one resolves to the active one. A label held only by retired agents fails with `AGENT_RETIRED`, as the stable name would.

A label held only by deregistered agents is not found. Deregistration is permanent, and the error must not point at an identity that can never receive mail.

### Resolution and delivery

- One candidate resolves to that agent's stable identity.
- No candidate fails with `RECIPIENT_DISPLAY_NAME_NOT_FOUND`. The error carries the label.
- Several candidates fail with `RECIPIENT_DISPLAY_NAME_AMBIGUOUS`. The error carries the label and the candidates' stable names. Agent Mail must not choose a candidate on the caller's behalf.
- Any selector that fails stops the whole send before anything is written.
- A `display_name` selector never registers an agent.
- Each selector resolves once at the start of the send. From then on the send uses the stable identity, so a relabel during the send changes nothing about this message.
- An agent identified more than once, by strings or by selectors, is kept once. The first occurrence wins, in the order `to`, `cc`, `bcc`.

Resolved recipients remain subject to the existing contact, authorization, project-boundary and delivery rules. Display-name addressing grants no new permission and does not enable broadcast messaging.

Delivery and persisted message references use stable identities. A later display-name change must not redirect existing messages or their receipts.

### Operations

The selector is accepted by `send_message` and by `reply_message`.

`whois` gains lookup by display name. It takes a separate `display_name` argument, exclusive with `agent_name`, and returns every active agent in the caller's project that carries the label, under the comparison and candidate rules above.

The result is an object with two fields: `display_name`, the label that was looked up, and `matches`, a list. Each match is one profile, as `whois` returns it for that agent. `matches` is empty when no active agent carries the label. That is not an error, because the caller is asking whether the label is in use. Calls with `agent_name` keep their response shape.

All other tools keep stable names.

### CLI

- `am mail send` gains `--to-display-name` and `--cc-display-name`. Each may be repeated and takes one label whole, with no comma splitting.
- `--to` is no longer required when a display-name flag is present. Stable and display-name flags may be combined in one command.
- `am mail reply` gains `--to-display-name`.
- Existing flags keep their meaning.

### Backward compatibility

Clients using the existing interface retain the same behavior without configuration changes or knowledge of this fork:

- Existing string recipients, including project-address syntax, keep their meaning in `to`, `cc` and `bcc`, registration on the spot included.
- Existing defaults, response shapes and delivery behavior remain unchanged for existing requests.
- The MCP schema advertises the additional recipient forms. Existing-client compatibility must be checked against that schema extension as well as against server behavior.

## Acceptance criteria

1. Existing string-only requests and CLI commands retain their behavior, including successful delivery, errors, project addressing, registration on the spot, and responses.
2. A unique display-name selector delivers to the matched stable identity in each of `to`, `cc` and `bcc`.
3. Duplicate labels among active agents produce `RECIPIENT_DISPLAY_NAME_AMBIGUOUS` with the candidates' stable names; an absent label produces `RECIPIENT_DISPLAY_NAME_NOT_FOUND`.
4. When one agent's stable name equals another agent's display name, each explicit selector resolves only in its declared namespace.
5. Failure to resolve a selector never falls back to the other namespace and never registers an agent.
6. Objects with both selectors, with neither, or with another field are rejected.
7. A selector that differs from the stored label only in case or in surrounding whitespace resolves; one that differs in inner whitespace does not.
8. A label shared by a retired and an active agent resolves to the active one; a label held only by retired agents fails with `AGENT_RETIRED`.
9. A message in which one selector fails writes nothing and reaches nobody.
10. An agent named by a string and by a selector receives the message once.
11. Relabeling after delivery leaves message ownership, receipt ownership and historical delivery destinations unchanged, including after restart.
12. Existing authorization, contact and project-isolation restrictions remain effective for display-name recipients, including BCC confidentiality. A display-name selector does not find an agent in another project.
13. `whois` by display name returns all active matches in the caller's project and an empty list for an unused label; `whois` by stable name is unchanged.
14. `am mail send` and `am mail reply` deliver through the display-name flags, including a label that contains a comma.
15. Existing-client conformance and the additional MCP and CLI addressing cases have passing evidence before the follow-up is considered complete.
