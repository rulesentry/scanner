import { describe, expect, it } from 'vitest';
import { scan } from './engine';
import rulesData from './rules.json';
import type { Rule, Span } from './redact';

const rules = rulesData as Rule[];
const rule = (id: string) => rules.filter((r) => r.id === id);
const run = (text: string, subset: Rule[] = rules): Span[] =>
  JSON.parse(scan(text, JSON.stringify(subset))) as Span[];
const custom = (id: string, pattern: string): Rule => ({
  id, name: id, kind: 'regex', pattern, replacement: `[${id}]`,
});

describe('overlap resolution', () => {
  it('keeps the earliest start, then the longest, never overlapping, sorted', () => {
    const spans = run('abcdef', [custom('cd', 'cd'), custom('bcd', 'bcd'), custom('bc', 'bc'), custom('ef', 'ef')]);
    expect(spans).toEqual([
      { start: 1, end: 4, ruleId: 'bcd' },
      { start: 4, end: 6, ruleId: 'ef' },
    ]);
  });

  it('a card number wins over a shorter phone match at the same start', () => {
    expect(run('pay 4539 1488 0343 6467 now')).toEqual([{ start: 4, end: 23, ruleId: 'card' }]);
  });

  it('an IBAN wins over a phone match inside it', () => {
    expect(run('GB82 WEST 1234 5698 7654 32')).toEqual([{ start: 0, end: 27, ruleId: 'iban' }]);
  });
});

describe('offsets', () => {
  it('are UTF-16 code units, end exclusive', () => {
    const text = '😀 mail jane@example.com';
    const [span] = run(text, rule('email'));
    expect(span).toEqual({ start: 8, end: 24, ruleId: 'email' });
    expect(text.slice(span.start, span.end)).toBe('jane@example.com');
  });
});

describe('rules passed in', () => {
  it('a rule not passed in finds nothing', () => {
    const text = 'jane@example.com +44 20 7946 0958';
    expect(run(text, rule('phone')).map((s) => s.ruleId)).toEqual(['phone']);
    expect(run(text, [])).toEqual([]);
  });

  it('an invalid pattern throws naming the rule id', () => {
    expect(() => scan('x', JSON.stringify([custom('broken', '(')]))).toThrow(/broken/);
  });
});

describe('the four v1 rules', () => {
  const cases: [string, string, string][] = [
    ['email', 'jane.doe@example.co.uk', 'jane.doe@example'],
    ['phone', '+44 20 7946 0958', '12 34'],
    ['card', '4539-1488-0343-6467', '4539 1488 03'],
    ['iban', 'GB82 WEST 1234 5698 7654 32', 'GB82 WEST'],
  ];
  for (const [id, good, bad] of cases) {
    it(`${id} matches a good sample whole`, () => {
      expect(run(`x ${good} y`, rule(id))).toEqual([{ start: 2, end: 2 + good.length, ruleId: id }]);
    });
    it(`${id} ignores a bad sample`, () => {
      expect(run(`x ${bad} y`, rule(id))).toEqual([]);
    });
  }
});
