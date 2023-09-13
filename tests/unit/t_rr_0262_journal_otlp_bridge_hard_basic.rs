//! Integration test for `RR-0262` (basic).
//! Journal OTLP bridge harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0262_journal_otlp_bridge_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x0d];
    let first = relayring::capabilities::rr_0262_journal_otlp_bridge_hard::evaluate(fixture).expect("RR-0262: Journal OTLP bridge harden index v12");
    let second = relayring::capabilities::rr_0262_journal_otlp_bridge_hard::evaluate(fixture).expect("RR-0262: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0262: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0262: window consumes the whole buffer");
}
