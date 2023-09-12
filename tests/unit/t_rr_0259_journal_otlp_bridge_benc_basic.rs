//! Integration test for `RR-0259` (basic).
//! Journal OTLP bridge benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0259_journal_otlp_bridge_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let first = relayring::capabilities::rr_0259_journal_otlp_bridge_benc::evaluate(fixture).expect("RR-0259: Journal OTLP bridge benchmark reporter v9");
    let second = relayring::capabilities::rr_0259_journal_otlp_bridge_benc::evaluate(fixture).expect("RR-0259: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0259: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0259: stats visits every byte");
}
