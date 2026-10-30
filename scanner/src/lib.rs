//! RuleSentry scanner: finds the spans of `text` that match the enabled rules.
//!
//! The pure logic lives in [`scan_spans`] so it is tested natively; [`scan`] is the thin
//! `wasm-bindgen` export the page calls.

use std::fmt;

use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// A rule as the page sends it. Unknown fields are ignored.
#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub kind: String,
    pub pattern: String,
    #[serde(default)]
    pub replacement: String,
}

/// A matched span: UTF-16 code unit offsets into the text, `end` exclusive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub rule_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanError {
    /// The rules JSON could not be parsed.
    InvalidRules(String),
    /// A rule's `kind` is not supported (v1 supports only `"regex"`).
    UnsupportedKind { rule_id: String, kind: String },
    /// A rule's pattern does not compile.
    InvalidPattern { rule_id: String, message: String },
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::InvalidRules(m) => write!(f, "invalid rules JSON: {m}"),
            ScanError::UnsupportedKind { rule_id, kind } => {
                write!(f, "rule \"{rule_id}\": unsupported kind \"{kind}\"")
            }
            ScanError::InvalidPattern { rule_id, message } => {
                write!(f, "rule \"{rule_id}\": invalid pattern: {message}")
            }
        }
    }
}

impl std::error::Error for ScanError {}

/// Post-match check applied by rule id. Every match passes in v1; the card rule's Luhn test
/// (#4) goes here.
fn passes_post_check(_rule_id: &str, _matched: &str) -> bool {
    true
}

/// Parses the rules JSON the page sends.
pub fn parse_rules(rules_json: &str) -> Result<Vec<Rule>, ScanError> {
    serde_json::from_str(rules_json).map_err(|e| ScanError::InvalidRules(e.to_string()))
}

/// Scans `text` with `rules`. Spans are sorted by `start` and never overlap: the earliest start
/// wins, then the longest match, then the rule listed first. Any invalid rule fails the whole
/// scan; there is no partial result.
pub fn scan_spans(text: &str, rules: &[Rule]) -> Result<Vec<Span>, ScanError> {
    let compiled = rules
        .iter()
        .map(|rule| {
            if rule.kind != "regex" {
                return Err(ScanError::UnsupportedKind {
                    rule_id: rule.id.clone(),
                    kind: rule.kind.clone(),
                });
            }
            Regex::new(&rule.pattern)
                .map(|re| (rule, re))
                .map_err(|e| ScanError::InvalidPattern {
                    rule_id: rule.id.clone(),
                    message: e.to_string(),
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Candidates in byte offsets: (start, end, rule index).
    let mut candidates: Vec<(usize, usize, usize)> = Vec::new();
    for (index, (rule, re)) in compiled.iter().enumerate() {
        for m in re.find_iter(text) {
            if !m.is_empty() && passes_post_check(&rule.id, m.as_str()) {
                candidates.push((m.start(), m.end(), index));
            }
        }
    }
    candidates.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)).then(a.2.cmp(&b.2)));

    let utf16 = Utf16Offsets::new(text);
    let mut spans = Vec::new();
    let mut taken_to = 0;
    for (start, end, index) in candidates {
        if start < taken_to {
            continue;
        }
        taken_to = end;
        spans.push(Span {
            start: utf16.at(start),
            end: utf16.at(end),
            rule_id: compiled[index].0.id.clone(),
        });
    }
    Ok(spans)
}

/// Maps byte offsets (on char boundaries) to UTF-16 code unit offsets.
struct Utf16Offsets {
    /// `(byte offset, utf16 offset)` of every char start, plus the end of the text.
    table: Vec<(usize, usize)>,
}

impl Utf16Offsets {
    fn new(text: &str) -> Self {
        let mut table = Vec::with_capacity(text.len() + 1);
        let mut units = 0;
        for (byte, ch) in text.char_indices() {
            table.push((byte, units));
            units += ch.len_utf16();
        }
        table.push((text.len(), units));
        Utf16Offsets { table }
    }

    fn at(&self, byte: usize) -> usize {
        match self.table.binary_search_by_key(&byte, |&(b, _)| b) {
            Ok(i) => self.table[i].1,
            // Regex matches always end on char boundaries; fall back to the char before.
            Err(i) => self.table[i.saturating_sub(1)].1,
        }
    }
}

/// `scan(text, rulesJson) -> string`: JSON `[{ "start": n, "end": n, "ruleId": "email" }]`.
#[wasm_bindgen]
pub fn scan(text: &str, rules_json: &str) -> Result<String, JsError> {
    let rules = parse_rules(rules_json).map_err(|e| JsError::new(&e.to_string()))?;
    let spans = scan_spans(text, &rules).map_err(|e| JsError::new(&e.to_string()))?;
    serde_json::to_string(&spans).map_err(|e| JsError::new(&e.to_string()))
}

#[cfg(test)]
mod tests;
