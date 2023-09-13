//! Integration test for `RR-0261` (basic).
//! Journal OTLP bridge extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0261_journal_otlp_bridge_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x0c];
    let first = relayring::capabilities::rr_0261_journal_otlp_bridge_exte::evaluate(fixture).expect("RR-0261: Journal OTLP bridge extend codec v11");
    let second = relayring::capabilities::rr_0261_journal_otlp_bridge_exte::evaluate(fixture).expect("RR-0261: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0261: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0261: window consumes the whole buffer");
}
