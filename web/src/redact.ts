export interface Rule {
  id: string;
  name: string;
  kind: string;
  pattern: string;
  replacement: string;
}

export interface Span {
  start: number;
  end: number;
  ruleId: string;
}

export type Segment = { text: string } | { text: string; ruleId: string };

/** Splits text into plain runs and matched runs, in order. Spans must be sorted and disjoint. */
export function segments(text: string, spans: Span[]): Segment[] {
  const out: Segment[] = [];
  let cursor = 0;
  for (const s of spans) {
    if (s.start > cursor) out.push({ text: text.slice(cursor, s.start) });
    out.push({ text: text.slice(s.start, s.end), ruleId: s.ruleId });
    cursor = s.end;
  }
  if (cursor < text.length) out.push({ text: text.slice(cursor) });
  return out;
}

/** The text with each span replaced by its rule's replacement string. */
export function replaceSpans(text: string, spans: Span[], rules: Rule[]): string {
  const byId = new Map(rules.map((r) => [r.id, r]));
  return segments(text, spans)
    .map((seg) => ('ruleId' in seg ? (byId.get(seg.ruleId)?.replacement ?? '') : seg.text))
    .join('');
}

/** Matches per rule id. */
export function countByRule(spans: Span[]): Map<string, number> {
  const counts = new Map<string, number>();
  for (const s of spans) counts.set(s.ruleId, (counts.get(s.ruleId) ?? 0) + 1);
  return counts;
}
