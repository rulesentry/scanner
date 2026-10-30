use super::*;

const RULES_V1: &str = include_str!("../rules.v1.json");

fn rules(ids: &[&str]) -> Vec<Rule> {
    parse_rules(RULES_V1)
        .unwrap()
        .into_iter()
        .filter(|r| ids.contains(&r.id.as_str()))
        .collect()
}

fn all_rules() -> Vec<Rule> {
    rules(&["email", "phone", "card", "iban"])
}

/// Spans as `(matched text, rule id)`, slicing by UTF-16 offsets as the page does.
fn found(text: &str, rule_set: &[Rule]) -> Vec<(String, String)> {
    let units: Vec<u16> = text.encode_utf16().collect();
    scan_spans(text, rule_set)
        .unwrap()
        .into_iter()
        .map(|s| (String::from_utf16(&units[s.start..s.end]).unwrap(), s.rule_id))
        .collect()
}

fn one(matched: &str, rule_id: &str) -> Vec<(String, String)> {
    vec![(matched.to_string(), rule_id.to_string())]
}

#[test]
fn fixture_has_the_four_v1_rules() {
    let r = parse_rules(RULES_V1).unwrap();
    let ids: Vec<_> = r.iter().map(|r| (r.id.as_str(), r.replacement.as_str())).collect();
    assert_eq!(
        ids,
        [("email", "[email]"), ("phone", "[phone]"), ("card", "[card]"), ("iban", "[iban]")]
    );
}

#[test]
fn email_good_and_bad() {
    let r = rules(&["email"]);
    assert_eq!(found("mail jane.doe+x@example.co.uk now", &r), one("jane.doe+x@example.co.uk", "email"));
    assert!(found("write to jane@localhost or @example.com", &r).is_empty());
}

#[test]
fn phone_good_and_bad() {
    let r = rules(&["phone"]);
    assert_eq!(found("call +44 20 7946 0958 today", &r), one("+44 20 7946 0958", "phone"));
    assert_eq!(found("office (555) 123-4567.", &r), one("(555) 123-4567", "phone"));
    assert_eq!(found("ring 020 7946 0958", &r), one("020 7946 0958", "phone"));
    assert!(found("in 2024 we sold 12345 units", &r).is_empty());
}

#[test]
fn card_good_and_bad() {
    let r = rules(&["card"]);
    assert_eq!(found("card 4111 1111 1111 1111 exp", &r), one("4111 1111 1111 1111", "card"));
    assert_eq!(found("card 5500-0000-0000-0004", &r), one("5500-0000-0000-0004", "card"));
    assert!(found("order 123456789012 shipped", &r).is_empty());
}

#[test]
fn iban_good_and_bad() {
    let r = rules(&["iban"]);
    assert_eq!(found("pay GB82 WEST 1234 5698 7654 32 now", &r), one("GB82 WEST 1234 5698 7654 32", "iban"));
    assert_eq!(found("DE89370400440532013000", &r), one("DE89370400440532013000", "iban"));
    assert!(found("see GB82 for details", &r).is_empty());
}

#[test]
fn offsets_are_utf16_code_units() {
    let r = rules(&["email"]);
    // "😀" is two UTF-16 units (four bytes); "é" is one unit (two bytes).
    let text = "😀 café a@b.io";
    let spans = scan_spans(text, &r).unwrap();
    assert_eq!(spans, vec![Span { start: 8, end: 14, rule_id: "email".into() }]);
    assert_eq!(found(text, &r), one("a@b.io", "email"));
}

#[test]
fn card_beats_phone_on_the_same_digits() {
    let text = "pay 4111 1111 1111 1111 and call 020 7946 0958";
    assert_eq!(
        found(text, &all_rules()),
        vec![
            ("4111 1111 1111 1111".to_string(), "card".to_string()),
            ("020 7946 0958".to_string(), "phone".to_string()),
        ]
    );
}

#[test]
fn spans_are_sorted_and_never_overlap() {
    let text = "x@y.com GB82 WEST 1234 5698 7654 32 +44 20 7946 0958 4111111111111111";
    let spans = scan_spans(text, &all_rules()).unwrap();
    let ids: Vec<_> = spans.iter().map(|s| s.rule_id.as_str()).collect();
    assert_eq!(ids, ["email", "iban", "phone", "card"]);
    assert!(spans.windows(2).all(|w| w[0].end <= w[1].start));
}

#[test]
fn disabled_rules_are_absent_from_the_input() {
    let text = "a@b.io and +44 20 7946 0958";
    assert_eq!(found(text, &rules(&["phone"])), one("+44 20 7946 0958", "phone"));
}

#[test]
fn invalid_pattern_names_the_rule_and_returns_nothing() {
    let json = r#"[
        {"id":"email","name":"Email","kind":"regex","pattern":"a@b","replacement":"[email]"},
        {"id":"broken","name":"Broken","kind":"regex","pattern":"(unclosed","replacement":"[x]"}
    ]"#;
    let err = scan_spans("a@b", &parse_rules(json).unwrap()).unwrap_err();
    assert!(matches!(&err, ScanError::InvalidPattern { rule_id, .. } if rule_id == "broken"));
    assert!(err.to_string().contains("\"broken\""));
}

#[test]
fn unsupported_kind_is_an_error() {
    let json = r#"[{"id":"x","name":"X","kind":"dictionary","pattern":"a","replacement":""}]"#;
    let err = scan_spans("a", &parse_rules(json).unwrap()).unwrap_err();
    assert!(err.to_string().contains("\"x\""));
}

#[test]
fn empty_text_returns_no_spans() {
    assert!(scan_spans("", &all_rules()).unwrap().is_empty());
}

#[test]
fn spans_serialize_with_rule_id_in_camel_case() {
    let spans = scan_spans("a@b.io", &rules(&["email"])).unwrap();
    assert_eq!(
        serde_json::to_string(&spans).unwrap(),
        r#"[{"start":0,"end":6,"ruleId":"email"}]"#
    );
}
