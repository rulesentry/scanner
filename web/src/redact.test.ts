import { describe, expect, it } from 'vitest';
import { countByRule, replaceSpans, segments, type Rule, type Span } from './redact';
import rulesData from './rules.json';

const rules = rulesData as Rule[];
const text = 'Mail a@b.co or call +44 20 7946 0958.';
const spans: Span[] = [
  { start: 5, end: 11, ruleId: 'email' },
  { start: 20, end: 36, ruleId: 'phone' },
];

describe('replaceSpans', () => {
  it('replaces each span with its rule replacement', () => {
    expect(replaceSpans(text, spans, rules)).toBe('Mail [email] or call [phone].');
  });

  it('leaves text without spans unchanged', () => {
    expect(replaceSpans(text, [], rules)).toBe(text);
  });
});

describe('segments', () => {
  it('splits into plain and matched runs that join back to the text', () => {
    const segs = segments(text, spans);
    expect(segs.map((s) => s.text).join('')).toBe(text);
    expect(segs.filter((s) => 'ruleId' in s).map((s) => s.text)).toEqual(['a@b.co', '+44 20 7946 0958']);
  });
});

describe('countByRule', () => {
  it('counts spans per rule', () => {
    expect(Object.fromEntries(countByRule([...spans, spans[0]]))).toEqual({ email: 2, phone: 1 });
  });
});
