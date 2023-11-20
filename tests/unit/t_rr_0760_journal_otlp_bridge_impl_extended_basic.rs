//! Integration test for `RR-0760` (basic).
//! Extended: Journal OTLP bridge implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0760_journal_otlp_bridge_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03];
    let first = relayring::capabilities::rr_0760_journal_otlp_bridge_impl_extended::evaluate(fixture).expect("RR-0760: Extended: Journal OTLP bridge implement pipeline v10");
    let second = relayring::capabilities::rr_0760_journal_otlp_bridge_impl_extended::evaluate(fixture).expect("RR-0760: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0760: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0760: stats visits every byte");
}
