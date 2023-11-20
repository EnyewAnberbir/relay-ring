//! Integration test for `RR-0752` (basic).
//! Extended: Journal OTLP bridge harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0752_journal_otlp_bridge_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf7, 0xf9];
    let first = relayring::capabilities::rr_0752_journal_otlp_bridge_hard_extended::evaluate(fixture).expect("RR-0752: Extended: Journal OTLP bridge harden index v2");
    let second = relayring::capabilities::rr_0752_journal_otlp_bridge_hard_extended::evaluate(fixture).expect("RR-0752: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0752: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0752: window consumes the whole buffer");
}
