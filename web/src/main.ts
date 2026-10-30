import './style.css';
import { scan } from './engine';
import rulesData from './rules.json';
import { countByRule, replaceSpans, segments, type Rule, type Span } from './redact';

const rules = rulesData as Rule[];
const enabled = new Map(rules.map((r) => [r.id, true]));

const paste = document.querySelector<HTMLTextAreaElement>('#paste')!;
const output = document.querySelector<HTMLDivElement>('#output')!;
const strip = document.querySelector<HTMLElement>('#strip')!;
const stripEnd = document.querySelector<HTMLElement>('#strip-end')!;
const summary = document.querySelector<HTMLElement>('#summary')!;
const copyButton = document.querySelector<HTMLButtonElement>('#copy')!;
const copyStatus = document.querySelector<HTMLElement>('#copy-status')!;

const names = new Map(rules.map((r) => [r.id, r.name]));
const rows = new Map<string, { row: HTMLElement; toggle: HTMLButtonElement; count: HTMLElement }>();

for (const rule of rules) {
  const row = document.createElement('label');
  row.className = 'rule';
  const toggle = document.createElement('button');
  toggle.type = 'button';
  toggle.setAttribute('role', 'switch');
  toggle.setAttribute('aria-label', rule.name);
  const name = document.createElement('span');
  name.className = 'name';
  name.textContent = rule.name;
  const count = document.createElement('span');
  count.className = 'count';
  row.append(toggle, name, count);
  strip.insertBefore(row, stripEnd);
  rows.set(rule.id, { row, toggle, count });
  toggle.addEventListener('click', () => {
    enabled.set(rule.id, !enabled.get(rule.id));
    render();
  });
}

let redactSpans: Span[] = [];

function runScan(subset: Rule[]): Span[] {
  return JSON.parse(scan(paste.value, JSON.stringify(subset))) as Span[];
}

function render(): void {
  const on = rules.filter((r) => enabled.get(r.id));
  redactSpans = runScan(on);
  const allCounts = countByRule(runScan(rules));
  const onCounts = countByRule(redactSpans);

  let shown = 0;
  for (const rule of rules) {
    const isOn = enabled.get(rule.id) === true;
    const { row, toggle, count } = rows.get(rule.id)!;
    const n = (isOn ? onCounts : allCounts).get(rule.id) ?? 0;
    if (!isOn) shown += n;
    row.classList.toggle('off', !isOn);
    toggle.className = `sw ${isOn ? 'on' : 'off'}`;
    toggle.setAttribute('aria-checked', String(isOn));
    count.textContent = String(n);
  }
  const hidden = redactSpans.length;
  summary.textContent = `${hidden} ${hidden === 1 ? 'match' : 'matches'} hidden, ${shown} shown`;

  const nodes = segments(paste.value, redactSpans).map((seg) => {
    if (!('ruleId' in seg)) return document.createTextNode(seg.text);
    const bar = document.createElement('span');
    bar.className = 'bar';
    bar.title = names.get(seg.ruleId) ?? seg.ruleId;
    bar.textContent = seg.text;
    return bar;
  });
  output.replaceChildren(...nodes);
}

let statusTimer: number | undefined;
function showStatus(message: string): void {
  copyStatus.textContent = message;
  window.clearTimeout(statusTimer);
  statusTimer = window.setTimeout(() => (copyStatus.textContent = ''), 2000);
}

copyButton.addEventListener('click', () => {
  const text = replaceSpans(paste.value, redactSpans, rules);
  navigator.clipboard.writeText(text).then(
    () => showStatus('Copied'),
    () => showStatus('Copy failed'),
  );
});

paste.addEventListener('input', render);
render();
