# GDPR redaction requirements and the rule catalog (preliminary)

**Status: preliminary, 2026-10-30.** Written from research that was stopped early. Not yet covered:
the CJEU judgments were not read in full (Breyer, Scania and SRB rest on secondary commentary); the
EDPB 01/2025 and 02/2026 PDFs and WP29 Opinion 05/2014 were not read in full (their points come from
the EDPB pages and law-firm summaries); several check-digit algorithms are marked "unverified" in the
catalog below; CNIL guidance was found only in its 2020 French edition; the effect of the UK Data (Use
and Access) Act 2025 on ICO guidance is still "under review"; no per-language word lists were looked
at.

The GDPR covers any information about an identified or identifiable person, whether identified
directly or indirectly (Art. 4(1)). Names, identification numbers, location data and online
identifiers are all named as examples. Data stops being personal data only when, taking account of
"all the means reasonably likely to be used", nobody can identify the person any more (Recital 26).
Pseudonymised data, where a key or other extra information lets someone re-attribute it, stays
personal data (Art. 4(5), Recital 26). However, the CJEU ruled in 2025 that identifiability is judged
from where each holder stands (EDPS v SRB, C-413/23 P). Art. 9 covers special categories of data and
Art. 10 covers criminal data. Regulators agree that removing direct identifiers is masking or
minimisation, not anonymisation: the text left behind can still identify someone.

For RuleSentry Scanner, this means the page is a data-minimisation aid (Art. 5(1)(c), Art. 25). It
is not an anonymiser, and its output should never be called anonymous. The catalog adds the most
value with identifiers that have a fixed format and a check digit: national IDs, health numbers,
online identifiers and machine-readable document lines. Those can be caught by regex plus a check
function. Names, street addresses and special-category facts cannot, because they need dictionaries
or models. One finding goes beyond the catalog: in the current page, the redaction bars keep the
original text in the page itself (see "What the browser-only design satisfies").

This document informs the catalog for #8. It is not legal advice.

## Scope

- **What the tool is.** A single web page. The user pastes text, regex rules find values, the page
  shows a redacted copy, and "Copy redacted" copies a version with category placeholders such as
  `[email]`. The scan runs in WebAssembly inside the browser. There is no server and no telemetry
  (see [SPEC.md](../SPEC.md)).
- **What it is not.** It is not a compliance guarantee, an anonymisation service or a records
  system. It does not check context, decide whether something may lawfully be disclosed, or keep any
  audit trail.
- **v1 rules.** `email`, `phone`, `card` (with Luhn) and `iban`.

## The legal frame

Sources: the regulation text on gdpr-info.eu. EUR-Lex refused automated fetches.

### Personal data, Art. 4(1) and Recital 30

- Personal data means "any information relating to an identified or identifiable natural person".
  A person can be identified "directly or indirectly".
- The examples given are "a name", "an identification number", "location data", "an online
  identifier", and factors specific to the person's physical, physiological, genetic, mental,
  economic, cultural or social identity. The list is illustrative, not closed.
- Recital 30 names "internet protocol addresses, cookie identifiers or other identifiers such as
  radio frequency identification tags". These can identify a person, especially when combined with
  other data.

### Anonymous and pseudonymised data, Recital 26 and Art. 4(5)

- The test is "all the means reasonably likely to be used", such as singling out, by the controller
  or another person. It weighs the cost, the time needed and the technology available.
- Data "rendered anonymous" so that the person is no longer identifiable falls outside the GDPR.
- Under Art. 4(5), pseudonymised data can be attributed to a person only with additional information
  that is "kept separately" under technical and organisational measures. It remains personal data
  for whoever can attribute it (Recital 26). Recitals 28 and 29 encourage pseudonymisation as a way
  to reduce risk.
- **Case law on identifiability.** All three holdings below come from secondary commentary; the
  judgments themselves were not read.
  - **Breyer, C-582/14 (19 Oct 2016).** A dynamic IP address is personal data for a website
    operator that has legal means to identify the user, with the provider's help, at reasonable
    effort.
  - **Scania, C-319/22 (9 Nov 2023).** A VIN is not personal data in isolation, but it is for a
    party that can reasonably link it to the keeper.
  - **EDPS v SRB, C-413/23 P (4 Sep 2025).** Pseudonymised data is not personal data for every
    holder in every case: identifiability is judged per holder. Opinions in free text are personal
    data of their author. The duty to inform people about who receives their data is still judged
    from the sending controller's side.

### Special categories and criminal data, Art. 9, Art. 10, Art. 4(13)-(15)

- Art. 9(1) lists the special categories:
  - data revealing racial or ethnic origin, political opinions, religious or philosophical beliefs,
    or trade union membership;
  - genetic data;
  - biometric data used to uniquely identify a person;
  - data concerning health;
  - data concerning a person's sex life or sexual orientation.
- Art. 4(13) to (15) define genetic, biometric and health data. Health data includes health care
  services that reveal a person's health status.
- Art. 10 covers data on criminal convictions and offences. It may be processed only under official
  authority, or where law authorises it with safeguards.
- These categories are about meaning: a diagnosis, a faith, a conviction. Regex can only catch
  structured identifiers attached to them, such as an NHS number or an ICD-10 code.

### National identifiers, Art. 87

- Member States may set conditions for processing "a national identification number or any other
  identifier of general application".
- These numbers are regulated nationally, persist for a lifetime, and often carry a check digit.
  That makes them the strongest case for new rules.

### Minimisation, by design, security

- Art. 5(1)(c): data must be "adequate, relevant and limited to what is necessary".
- Art. 25(1) names pseudonymisation as one way to build data minimisation into processing.
  Art. 25(2) requires that, by default, only the data needed for each purpose is processed.
- Art. 32(1)(a) lists pseudonymisation and encryption among security measures.
- Art. 2(2)(c) exempts purely personal or household use. It does not cover a professional user
  redacting work text.

### What counts as an identifier in free text

- **Direct identifiers** point to one person on their own. Examples: name, email, phone, national
  ID, passport or licence number, IBAN or account number, card number, customer or employee number,
  exact address.
- **Quasi-identifiers** are weak alone but identify in combination: date of birth, postcode, gender,
  job title, employer, event dates, vehicle registration or VIN, IP and device identifiers. This is
  the "indirectly" limb of Art. 4(1) and the singling-out test of Recital 26.

## What regulators say about redaction

| Source | Date | Status |
|---|---|---|
| WP29 Opinion 05/2014 on anonymisation techniques (WP216) | adopted 10 Apr 2014 | dated, pre-GDPR; reported to be replaced by EDPB 02/2026 |
| EDPB Guidelines 01/2025 on pseudonymisation | adopted 16 Jan 2025 for consultation | draft: v1 is the only version listed as of 2026-10-30 |
| EDPB Guidelines 02/2026 on anonymisation | adopted 7 Jul 2026 for consultation | draft: consultation to 30 Oct 2026; PDF not read, content from summaries |
| ICO anonymisation and pseudonymisation guidance | final 28 Mar 2025 | under review after the Data (Use and Access) Act 2025 |
| ICO "Disclosing documents to the public securely" | 31 Jul 2025 | final, under review; replaces the 2020 "How to disclose information safely" |
| ICO right of access: information about other individuals | updated 8 Dec 2025 | final |
| CNIL "L'anonymisation de données personnelles" | 19 May 2020 | dated, French only |
| AEPD and EDPS "10 misunderstandings related to anonymisation" | 27 Apr 2021 | joint paper, non-binding |

### Redaction, pseudonymisation and anonymisation

- **Redaction** is "permanently removing information from a copy of a document" (ICO). It removes
  values. Whether the result is anonymous depends on what is left.
- **Pseudonymisation** replaces identifiers with new ones that can be reversed with separately held
  extra information (EDPB 01/2025, draft). It keeps the data personal.
  - A placeholder like `[email]` with no mapping table is not pseudonymisation in the Art. 4(5)
    sense, because there is nothing to re-attribute it with.
  - Numbered placeholders (`[email-1]`) with a mapping kept only in the browser would come closer to
    pseudonymisation. They would also keep links between mentions of the same value.
- **Anonymisation** must pass three tests:
  - **Singling out.** WP216 calls it singling out; the EDPB 02/2026 draft calls it record isolation.
  - **Linkability.** Records about the same person can be linked.
  - **Inference.** An attribute can be deduced from other attributes.

  The CNIL uses the same three tests. The EDPB 02/2026 draft, as reported, says "removing names,
  numbers, or other identifiers will rarely be sufficient to establish anonymity". It also judges
  anonymity per holder, following SRB.
- The ICO states it directly: "Masking alone is not considered an effective anonymisation
  technique", and "Simply removing direct identifiers from a dataset is insufficient".

### Residual context and re-identification

- The ICO says context can identify a person even without a name. Combinations of details (the
  "mosaic or jigsaw effect") and outliers can still single someone out.
- **Motivated intruder.** The ICO's test is a "reasonably competent" person with access to "the
  internet, libraries, public documents". The EDPB 02/2026 draft instead lists the entities that
  could reasonably attempt re-identification, from the recipients up to criminals.
- **Free text.** The ICO says quantitative techniques "are not generally applicable" to free text.
  Transcripts need identifiers removed, blurred or reworded, often with human judgement. The AEPD and
  EDPS paper lists "anonymisation can be fully automated" among its misunderstandings.
- **Subject access requests.** Removing a name may not protect a third party if the requester can
  still work out who it is. Consider what the requester already knows (ICO right of access).
- **Residual risk.** Risk is never zero. The ICO asks for it to be reduced and the decision
  recorded. The EDPB draft asks for risk that is "insignificant in reality".

### Safe redaction mechanics

- The ICO lists these methods as ineffective:
  - black boxes drawn over electronic text ("the information remains in the electronic file if it
    is only covered by a shape");
  - changing the text colour;
  - blurring.
- It recommends:
  - a visible marker such as "[redacted]";
  - redacting a copy and keeping the original;
  - review by a second person;
  - checking for hidden content such as metadata and hidden rows.

### Data minimisation

- The ICO says removing identifiers that are not needed is a minimisation duty, even when the result
  is not anonymous. This is the job RuleSentry does.

## What the browser-only design satisfies, and what it does not

**Satisfies:**

- **Minimisation at the point of sharing.** The user removes identifiers before text goes into a
  ticket, an email or another tool (Art. 5(1)(c), Art. 25).
- **No new processor.** The text never leaves the browser, so the operator of the page does not
  receive the personal data. This holds by design: no request after load (SPEC "Purpose").
- **A visible marker.** "Copy redacted" replaces each match with a category placeholder, which
  matches the ICO's advice to show where something was removed.
- **The copy, not the original.** The paste area holds the original. The output is a separate copy.

**Does not satisfy:**

- **The bars in the output pane still hold the original text.** `web/src/main.ts` renders each
  match as `<span class="bar">` with `textContent` set to the matched value. The bar hides the value
  only visually. Selecting and copying from the output pane (the SPEC says it is "selectable") and the
  accessibility tree still carry the value. This is the "black box over
  electronic text" failure the ICO describes. The bar should hold a placeholder or blank text, never
  the match. This is not catalog work and is recorded as a finding on #15.
- **The user still holds the original.** The redacted copy remains personal data for the user and
  their organisation, who have the source (Recital 26, SRB).
- **Context is not judged.** Names, job titles, rare events and quasi-identifier combinations stay
  in the text. The page cannot tell when what remains identifies someone. A human check is still
  needed (ICO, AEPD and EDPS).
- **Placeholders carry the category.** `[email]` reveals that an email was there, and `[iban]`
  reveals that a bank account was mentioned. That is a small but real disclosure. Categories should
  never be more specific than they need to be: no country, issuer or partial digits.
- **There is no record of what was redacted.** The ICO's practice for disclosure redaction (who,
  when, why) is out of scope for a page with no saved state.

## Identifier catalog

GDPR column:
- **ordinary**: Art. 4(1).
- **special**: Art. 9.
- **Art. 10**: criminal data.
- **Art. 87**: national identifier.

Regex column:
- **yes**: regex plus a check function gives good precision.
- **partly**: the format can be matched, but false positives stay high without context words or a
  table.
- **no**: needs a dictionary or a model.

"Unverified" means the check is from general knowledge and was not checked against a primary
source.

| Identifier class | GDPR | In v1 | Regex + check | Format and check notes |
|---|---|---|---|---|
| Email address | ordinary | yes | yes | `local@domain.tld` |
| Phone number | ordinary | yes | partly | 7 to 15 digits, no check digit |
| Payment card | ordinary | yes | yes | 13 to 19 digits, Luhn |
| IBAN | ordinary | yes | yes | ISO 13616, mod 97 |
| Person name | ordinary | no | no | needs NER or a gazetteer; all three reference tools use models |
| Street address | ordinary | no | no | free text; models in all three tools |
| UK postcode | ordinary (quasi) | no | yes | strict outward/inward shape, no check digit |
| US ZIP, DE PLZ | ordinary (quasi) | no | partly | bare 5 digits; needs address context |
| Date of birth | ordinary (quasi) | no | partly | calendar check; "DOB", "born", "né le" context |
| UK National Insurance number | Art. 87 | no | yes | `AA999999A`; HMRC prefix exclusions, suffix A to D, no check digit |
| UK NHS number | Art. 87 / special (health) | no | yes | 10 digits, mod 11 (10 = invalid) |
| French NIR | Art. 87 | no | yes | 13 digits + key `97 - (n mod 97)`; Corsica 2A/2B replaced before the mod |
| German Steuer-ID | Art. 87 | no | partly | 11 digits, ISO 7064 MOD 11,10, digit-repetition rule (unverified); bare digits |
| German Personalausweis | Art. 87 | no | partly | 9-character serial + 7-3-1 check (unverified) |
| Spanish DNI / NIE | Art. 87 | no | yes | letter = `TRWAGMYFPDXBNJZSQVHLCKE`[n mod 23]; NIE X/Y/Z become 0/1/2 |
| Italian codice fiscale | Art. 87 | no | yes | 16 chars, odd/even table check letter (unverified); omocodia |
| Dutch BSN | Art. 87 | no | partly | 9 digits, 11-proof (unverified); bare digits |
| Belgian national number | Art. 87 | no | yes | `YY.MM.DD-SSS.CC`, `97 - (n mod 97)`, "2" prefix for 2000+ (unverified) |
| Polish PESEL | Art. 87 | no | yes | 11 digits, weights 1-3-7-9 (unverified) |
| Swedish personnummer | Art. 87 | no | yes | `YYMMDD-NNNC`, Luhn (unverified) |
| Danish CPR | Art. 87 | no | partly | `DDMMYY-NNNN`; mod 11 dropped for some numbers since 2007; date check only |
| US SSN | Art. 87 | no | partly | `AAA-GG-SSSS`; SSA invalid ranges (000, 666, 900+); no check digit |
| Passport, MRZ line | Art. 87 | no | yes | ICAO 9303 7-3-1 check per field plus composite |
| Passport, bare number | Art. 87 | no | partly | 6 to 9 chars, no check digit; needs context |
| UK driving licence | Art. 87 | no | yes | 16 chars, encoded surname and birth date (sources conflict; check GOV.UK) |
| IPv4 / IPv6 address | ordinary (Recital 30) | no | yes | octet range; IPv6 parsed with `std::net` |
| MAC address | ordinary (Recital 30) | no | yes | 6 hex pairs |
| URL | ordinary (may carry IDs) | no | yes | TLD list |
| IMEI | ordinary (device) | no | partly | 15 digits, Luhn; collides with `card` |
| Vehicle plate (UK, FR, DE) | ordinary (Scania) | no | partly | no check digit; DE needs a district list |
| VIN | ordinary (Scania) | no | yes | 17 chars, no I/O/Q; position 9 check in North America only (49 CFR 565.15) |
| UK sort code + account | ordinary | no | partly | format only; the full check needs the Vocalink modulus table (changes; v9.00) |
| SWIFT/BIC | ordinary (bank, rarely personal) | no | partly | ISO 9362, positions 5-6 an ISO 3166 country; no check digit |
| US ABA routing | ordinary (bank) | no | yes | 9 digits, 3-7-1 mod 10 |
| Crypto address | ordinary | no | yes | Base58Check, bech32/bech32m, EIP-55 |
| ICD-10 code | special (health) | no | partly | `A00.0` shape; collides with other codes |
| Health, religion, union, sexual orientation, political, ethnicity terms | special | no | no | word lists only; flags a term, not a fact about a person |
| Criminal offence or conviction terms | Art. 10 | no | no | word lists only |

How the reference tools classify these, in short:
- **Microsoft Presidio.** Pattern, checksum and context recognizers for most national IDs, plus NER
  for PERSON, LOCATION and NRP.
- **Google Sensitive Data Protection.** The widest set of infoTypes, including FRANCE_NIR,
  NETHERLANDS_BSN_NUMBER, IMEI_HARDWARE_ID and SWIFT_CODE. It is also the only tool with special
  category term infoTypes (MEDICAL_TERM, RELIGIOUS_TERM, TRADE_UNION and others).
- **AWS Comprehend PII.** English and Spanish only. Its national IDs are UK, US, Canadian and Indian
  only, with no EU identifiers.

## Conclusions

### Candidate rules for #8, in priority order

1. `uk-nino`: regex with the HMRC prefix exclusions; no check digit, but the shape is distinctive.
2. `nhs-number`: 10 digits, mod 11 check.
3. `ipv4` / `ipv6`: regex, then a parse check (octets at most 255; `std::net` for IPv6).
4. `es-dni-nie`: regex plus the mod 23 letter check.
5. `fr-nir`: regex plus the 97-mod key, with the Corsica 2A/2B replacement.
6. `it-codice-fiscale`: regex plus the check letter (verify the tables before building).
7. `passport-mrz`: MRZ line regex plus the ICAO 7-3-1 checks.
8. `mac-address`: regex.
9. `uk-postcode`: strict regex.
10. `se-personnummer`, `pl-pesel`, `be-national-number`: regex plus their checks (verify the
    algorithms first).
11. `vin`: regex; the North American check raises confidence but does not reject a match.
12. `date-of-birth`: date regex with a calendar check, matched only next to a context word. This
    needs context-word support in the engine.
13. `nl-bsn`, `de-steuer-id`, `us-ssn`, `imei`: checked or ranged digits that need context words,
    and ordering against `card` for IMEI. Hold these until context words exist.

### Product implications

- **Names, street addresses and Art. 9/10 terms are out of reach for regex.** A "names" rule needs a
  dictionary or a model. That is a separate engine `kind`, not a pattern, and should be ruled on
  before it is promised.
- **Context words are the next engine feature.** Several identifiers (date of birth, SSN, BSN,
  Steuer-ID, IMEI, bare passport numbers, SWIFT/BIC) only become precise with a nearby keyword. That
  is a change to the rule format in the SPEC, not a new pattern.
- **Placeholders must not leak more than the category.** Use `[national-id]` or `[id-number]`, never
  `[fr-nir]` with digits kept. Never keep the last four digits. Numbered placeholders (`[email-1]`)
  would keep links between mentions inside a document and should be a deliberate choice, not a
  default.
- **The output bar must not hold the value.** Fix the page so the redacted pane never holds the
  matched text in the page itself (see above).
- **The page should not say "anonymised".** "Identifiers removed" or "redacted" is accurate. A
  one-line reminder that context can still identify a person fits the ICO and AEPD/EDPS position.
- **The allow-list in #5 matters more as the catalog grows.** Bare-digit and letter-digit rules
  (BSN, SWIFT/BIC, plates, postcodes) will over-redact product codes, version numbers and
  references. A per-term allow-list keeps precision without dropping the rule.
- **Overlap order needs a rule.** IMEI and card share Luhn and 13 to 19 digits. The current tie-break
  (earliest start, then longest) will pick by length. A catalog with overlapping classes needs an
  explicit priority.

## Sources

### Legal frame

- GDPR text, ELI: https://eur-lex.europa.eu/eli/reg/2016/679/oj
- Art. 4: https://gdpr-info.eu/art-4-gdpr/
- Art. 2: https://gdpr-info.eu/art-2-gdpr/
- Art. 5: https://gdpr-info.eu/art-5-gdpr/
- Art. 9: https://gdpr-info.eu/art-9-gdpr/
- Art. 10: https://gdpr-info.eu/art-10-gdpr/
- Art. 25: https://gdpr-info.eu/art-25-gdpr/
- Art. 32: https://gdpr-info.eu/art-32-gdpr/
- Art. 87: https://gdpr-info.eu/art-87-gdpr/
- Recital 26: https://gdpr-info.eu/recitals/no-26/
- Recital 28: https://gdpr-info.eu/recitals/no-28/
- Recital 29: https://gdpr-info.eu/recitals/no-29/
- Recital 30: https://gdpr-info.eu/recitals/no-30/
- Breyer, C-582/14: https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:62014CJ0582
- Breyer summary: https://www.thomashelbing.com/en/wissen/dsgvo-hub/rechtsprechung/1.4.37-eugh-breyer
- Scania, C-319/22: https://vbb.com/insights/cjeu-reiterates-relative-nature-of-what-is-personal-data-in-case-on-access-to-vehicle-spare-part-i
- Scania, C-319/22: https://datenrecht.ch/en/eugh-i-s-scania-rs-c%e2%80%91319-22-relativer-ansatz-bei-der-bestimmung-der-identifizierbarkeit/
- EDPS v SRB, C-413/23 P, OJ notice: https://eur-lex.europa.eu/eli/C/2025/5551/oj/eng
- EDPS v SRB: https://www.aoshearman.com/en/insights/ao-shearman-on-data/cjeu-clarifies-concept-of-personal-data-for-a-transfer-of-pseudonymised-data-to-third-parties
- EDPS v SRB: https://www.osborneclarke.com/insights/cjeu-clarifies-concepts-personal-and-pseudonymised-data-implications-data-sharing
- EDPS v SRB: https://www.cliffordchance.com/insights/resources/blogs/talking-tech/en/articles/2025/09/pseudonymized-data-after-edps-v-srb.html

### Regulator guidance

- WP29 Opinion 05/2014: https://ec.europa.eu/justice/article-29/documentation/opinion-recommendation/files/2014/wp216_en.pdf
- EDPB 01/2025, page: https://www.edpb.europa.eu/documents/guideline/guidelines-012025-on-pseudonymisation_en
- EDPB 01/2025, consultation: https://www.edpb.europa.eu/our-work-tools/documents/public-consultations/2025/guidelines-012025-pseudonymisation_en
- EDPB 01/2025, PDF: https://www.edpb.europa.eu/system/files/2025-01/edpb_guidelines_202501_pseudonymisation_en.pdf
- EDPB 01/2025, summary: https://www.hunton.com/insights/publications/edpb-advises-on-pseudonymisation-for-gdpr-compliance
- EDPB work programme 2026-2027: https://www.edpb.europa.eu/system/files/2026-02/edpb_work-programme_2026-2027_en.pdf
- EDPB 02/2026, consultation: https://www.edpb.europa.eu/public-consultations/guidelines-022026-on-anonymisation_fr
- EDPB 02/2026, PDF: https://www.edpb.europa.eu/system/files/2026-09/edpb_guidelines_202602_anonymisation_v1_en_0.pdf
- EDPB 02/2026, summary: https://www.edpb.europa.eu/system/files/2026-10/edpb-summary-102026-anonymisation_en.pdf
- EDPB 02/2026, commentary: https://www.stephensonharwood.com/insights/the-edpbs-guidelines-on-anonymisation-one-legal-standard-two-approaches-three-criteria/
- EDPB 02/2026, commentary: https://www.williamfry.com/knowledge/the-edpbs-anonymisation-standard-new-draft-guidelines/
- ICO anonymisation hub: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/data-sharing/anonymisation/
- ICO, effective anonymisation: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/data-sharing/anonymisation/how-do-we-ensure-anonymisation-is-effective/
- ICO, introduction: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/data-sharing/anonymisation/introduction-to-anonymisation/
- ICO, pseudonymisation: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/data-sharing/anonymisation/pseudonymisation/
- ICO final edition, commentary: https://www.twobirds.com/en/insights/2025/key-takeaways-from-the-ico’s-final-anonymisation-guidance
- ICO final edition, commentary: https://www.rpclegal.com/snapshots/data-protection/summer-2025/ico-publishes-new-guidance-on-anonymisation-and-pseudonymisation/
- ICO disclosing documents securely: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/security/disclosing-documents-to-the-public-securely/
- ICO redaction: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/security/disclosing-documents-to-the-public-securely/how-do-we-avoid-an-accidental-breach-when-redacting-information/
- ICO redaction checklist: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/security/disclosing-documents-to-the-public-securely/checklist-avoid-an-accidental-breach-when-redacting-documents/
- ICO hidden information checklist: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/security/disclosing-documents-to-the-public-securely/checklist-avoid-an-accidental-breach-when-personal-information-is-hidden-in-documents/
- ICO 2020 edition (dated): https://cy.ico.org.uk/media2/for-organisations/documents/2021/2618998/how-to-disclose-information-safely-20201224.pdf
- ICO right of access, other people: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/individual-rights/right-of-access/information-about-other-individuals
- ICO Part 3 right of access: https://ico.org.uk/for-organisations/law-enforcement/the-right-of-access-part-3-of-the-dpa-2018/what-should-we-do-if-the-part-3-request-involves-information-about-other-people/
- CNIL: https://www.cnil.fr/fr/lanonymisation-de-donnees-personnelles
- CNIL, language note: https://www.dataguidance.com/news/france-cnil-issues-guidance-anonymisation-personal-data
- AEPD and EDPS paper: https://edps.europa.eu/data-protection/our-work/publications/papers/aepd-edps-joint-paper-10-misunderstandings-related_en
- AEPD and EDPS paper, summary: https://dwfgroup.com/en/news-and-insights/insights/2021/4/dwf-data-protection-insights-april-2021

### Practice and catalog

- Presidio supported entities: https://presidio.dataprivacystack.org/supported_entities/
- Google infoTypes reference: https://docs.cloud.google.com/sensitive-data-protection/docs/infotypes-reference
- AWS Comprehend PII: https://docs.aws.amazon.com/comprehend/latest/dg/how-pii.html
- HMRC NIM39110 (NINO prefixes): https://www.gov.uk/hmrc-internal-manuals/national-insurance-manual/nim39110
- NHS number (Data Dictionary archive): https://archive.datadictionary.nhs.uk/DD%20Release%20May%202024/attributes/nhs_number.html
- BZSt Steuer-ID: https://www.bzst.de/DE/Privatpersonen/SteuerlicheIdentifikationsnummer/steuerlicheidentifikationsnummer_node.html
- ELSTER check-digit specification: https://download.elster.de/download/schnittstellen/Pruefung_der_Steuer_und_Steueridentifikatsnummer.pdf
- Spanish DNI/NIE control letter: https://www.interior.gob.es/opencms/es/servicios-al-ciudadano/tramites-y-gestiones/dni/calculo-del-digito-de-control-del-nif-nie/
- Danish CPR: https://admin.cpr.dk/media/12066/personnummeret-i-cpr.pdf
- US SSN randomization: https://www.ssa.gov/employer/randomizationfaqs.html
- VIN check digit, 49 CFR 565.15: https://www.ecfr.gov/current/title-49/subtitle-B/chapter-V/part-565/subpart-B/section-565.15
- Vocalink modulus checking: https://vocalink.com/tools/modulus-checking/
- Vocalink specification v9.00: https://www.vocalink.com/media/sdjd3qea/validating-account-numbers-uk-modulus-checking-v900.pdf
