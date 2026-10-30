# RuleSentry Scanner: product spec

The comps and the first build render from this file. Where it and good practice disagree, the
file changes and the issue says so.

## Purpose

A single page that takes pasted text, finds sensitive values in it with a small set of rules, and
shows a redacted copy the user can take away. Every byte of the scan happens in the browser: the
rules run in a WebAssembly module, and the page makes no network request after it has loaded.

## The page

One screen: a header, the rule strip under it, then two wide panes side by side. On a narrow
window the panes stack, paste area first.

- **"Nothing leaves this page"** as one quiet line in the header, always visible.
- **Rule strip.** One strip under the header. For each rule: its name, a toggle, and a plain count
  of matches in the current text. A rule switched off keeps its place and shows its count greyed;
  its matches are not redacted. The strip ends with a short summary, "N matches hidden, M shown".
- **Paste area (left pane).** A plain multi-line text field, placeholder "Paste text to scan".
  The scan re-runs on every edit.
- **Redacted output (right pane).** The pasted text with every match replaced by a redaction bar.
  Read-only, selectable.
- **Copy redacted**, below the panes at the bottom right. Copies the output, with each match
  replaced by the rule's `replacement` string, to the clipboard. It is the ONE filled button on
  the page; everything else is a toggle, a text field or a text button.

## Rules v1

| id | name | what it matches |
|---|---|---|
| `email` | Email address | `local@domain.tld` |
| `phone` | Phone number | international (`+44 20 7946 0958`) and common national forms, 7 to 15 digits |
| `card` | Card number | 13 to 19 digits, spaces or dashes allowed, **and** passes the Luhn check |
| `iban` | IBAN | country code, two check digits, up to 30 alphanumerics, spaces allowed |

A rule is data, not code:

```json
{ "id": "email", "name": "Email address", "kind": "regex",
  "pattern": "[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}",
  "replacement": "[email]" }
```

`kind` is `"regex"` in v1. A rule may carry a post-match check (the card rule's Luhn test) that
the engine applies by `id`. Overlapping matches resolve to the earliest start, then the longest.

## The WASM contract

```text
scan(text: string, rulesJson: string) -> string   // JSON: [{ "start": n, "end": n, "ruleId": "email" }]
```

- `start` and `end` are UTF-16 code unit offsets into `text`, end exclusive, so the page can slice
  the JavaScript string directly. Spans are sorted by `start` and never overlap.
- Only enabled rules are passed in; the page computes counts from the spans.
- An invalid pattern returns an error naming the rule id, never a partial result.
- Rust, `wasm-bindgen`, built with `wasm-pack build --target web`. Computed entirely in the
  browser.

## Out of scope

- File upload: v1 is paste only.
- The real RuleSentry rule catalog: v1 ships the four rules above.
- Any server: no API, no telemetry, no saved sessions.

## Design

- Light theme only, the system font stack, sentence case everywhere.
- One accent colour, `#1F5F8B`, used for the filled button, focus rings and active toggles.
- Redactions shown as solid black bars the width of the hidden text; hovering a bar shows the
  rule name.
- One filled button (Copy redacted). Counts are plain numbers, not badges.
- The header is a band in the accent with white text; the quiet line is in a pale tint of it.
- Page ground `#eef4f8`. The rule strip and the two panes are white cards with `#b9d2e5` borders.
- Section labels and counts in the accent. Redaction bars stay black.
- The picked comp is [`docs/comps/page.html`](comps/page.html) (comp B, ruling on #1, 2026-10-30).
