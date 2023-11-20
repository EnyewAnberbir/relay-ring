//! Integration test for `RR-0751` (basic).
//! Extended: Journal OTLP bridge extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0751_journal_otlp_bridge_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf6, 0xf8];
    let first = relayring::capabilities::rr_0751_journal_otlp_bridge_exte_extended::evaluate(fixture).expect("RR-0751: Extended: Journal OTLP bridge extend codec v1");
    let second = relayring::capabilities::rr_0751_journal_otlp_bridge_exte_extended::evaluate(fixture).expect("RR-0751: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0751: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0751: stats visits every byte");
}
