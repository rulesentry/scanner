// The scan engine, behind one module. This body is a TypeScript stand-in for the Rust WASM
// crate (#2); a later task swaps it for the WASM import. Nothing else in the app may know which
// engine runs: callers use only scan(text, rulesJson) and the JSON it returns.
//
// Contract (docs/SPEC.md): returns JSON [{start, end, ruleId}], UTF-16 offsets, end exclusive,
// sorted by start, never overlapping; overlaps resolve to the earliest start, then the longest.
// Only the rules passed in run. An invalid pattern throws an error naming the rule id.

interface EngineRule {
  id: string;
  kind: string;
  pattern: string;
}

interface EngineSpan {
  start: number;
  end: number;
  ruleId: string;
}

export function scan(text: string, rulesJson: string): string {
  const rules = JSON.parse(rulesJson) as EngineRule[];
  const found: EngineSpan[] = [];

  for (const rule of rules) {
    if (rule.kind !== 'regex') {
      throw new Error(`rule ${rule.id}: unsupported kind ${rule.kind}`);
    }
    let re: RegExp;
    try {
      re = new RegExp(rule.pattern, 'g');
    } catch (e) {
      throw new Error(`rule ${rule.id}: invalid pattern: ${(e as Error).message}`, { cause: e });
    }
    for (const m of text.matchAll(re)) {
      if (m[0].length === 0) continue;
      found.push({ start: m.index, end: m.index + m[0].length, ruleId: rule.id });
    }
  }

  found.sort((a, b) => a.start - b.start || b.end - a.end);

  const spans: EngineSpan[] = [];
  let cursor = 0;
  for (const span of found) {
    if (span.start < cursor) continue;
    spans.push(span);
    cursor = span.end;
  }
  return JSON.stringify(spans);
}
