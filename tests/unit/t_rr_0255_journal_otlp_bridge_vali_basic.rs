//! Integration test for `RR-0255` (basic).
//! Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0255_journal_otlp_bridge_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06];
    let first = relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(fixture).expect("RR-0255: Journal OTLP bridge validate resolver v5");
    let second = relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(fixture).expect("RR-0255: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0255: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0255: scanner should emit domain hints");
}
