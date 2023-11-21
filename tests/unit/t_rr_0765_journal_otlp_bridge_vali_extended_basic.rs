//! Integration test for `RR-0765` (basic).
//! Extended: Journal OTLP bridge validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0765_journal_otlp_bridge_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x08];
    let first = relayring::capabilities::rr_0765_journal_otlp_bridge_vali_extended::evaluate(fixture).expect("RR-0765: Extended: Journal OTLP bridge validate resolver v15");
    let second = relayring::capabilities::rr_0765_journal_otlp_bridge_vali_extended::evaluate(fixture).expect("RR-0765: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0765: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0765: scanner should emit domain hints");
}
