//! Integration test for `RR-0756` (basic).
//! Extended: Journal OTLP bridge export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0756_journal_otlp_bridge_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0xfd];
    let first = relayring::capabilities::rr_0756_journal_otlp_bridge_expo_extended::evaluate(fixture).expect("RR-0756: Extended: Journal OTLP bridge export adapter v6");
    let second = relayring::capabilities::rr_0756_journal_otlp_bridge_expo_extended::evaluate(fixture).expect("RR-0756: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0756: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0756: scanner should emit domain hints");
}
