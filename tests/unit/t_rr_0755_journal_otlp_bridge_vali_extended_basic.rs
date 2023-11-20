//! Integration test for `RR-0755` (basic).
//! Extended: Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0755_journal_otlp_bridge_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0xfc];
    let first = relayring::capabilities::rr_0755_journal_otlp_bridge_vali_extended::evaluate(fixture).expect("RR-0755: Extended: Journal OTLP bridge validate resolver v5");
    let second = relayring::capabilities::rr_0755_journal_otlp_bridge_vali_extended::evaluate(fixture).expect("RR-0755: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0755: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0755: window consumes the whole buffer");
}
