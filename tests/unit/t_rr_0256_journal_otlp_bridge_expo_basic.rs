//! Integration test for `RR-0256` (basic).
//! Journal OTLP bridge export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0256_journal_otlp_bridge_expo_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x07];
    let first = relayring::capabilities::rr_0256_journal_otlp_bridge_expo::evaluate(fixture).expect("RR-0256: Journal OTLP bridge export adapter v6");
    let second = relayring::capabilities::rr_0256_journal_otlp_bridge_expo::evaluate(fixture).expect("RR-0256: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0256: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0256: window consumes the whole buffer");
}
