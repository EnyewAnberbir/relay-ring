//! Integration test for `RR-0758` (basic).
//! Extended: Journal OTLP bridge refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0758_journal_otlp_bridge_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x01];
    let first = relayring::capabilities::rr_0758_journal_otlp_bridge_refa_extended::evaluate(fixture).expect("RR-0758: Extended: Journal OTLP bridge refactor mutator v8");
    let second = relayring::capabilities::rr_0758_journal_otlp_bridge_refa_extended::evaluate(fixture).expect("RR-0758: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0758: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0758: window consumes the whole buffer");
}
