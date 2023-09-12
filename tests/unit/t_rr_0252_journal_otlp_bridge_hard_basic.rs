//! Integration test for `RR-0252` (basic).
//! Journal OTLP bridge harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0252_journal_otlp_bridge_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03];
    let first = relayring::capabilities::rr_0252_journal_otlp_bridge_hard::evaluate(fixture).expect("RR-0252: Journal OTLP bridge harden index v2");
    let second = relayring::capabilities::rr_0252_journal_otlp_bridge_hard::evaluate(fixture).expect("RR-0252: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0252: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0252: scanner should emit domain hints");
}
