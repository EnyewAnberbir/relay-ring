//! Integration test for `RR-0754` (basic).
//! Extended: Journal OTLP bridge optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0754_journal_otlp_bridge_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0xfb];
    let first = relayring::capabilities::rr_0754_journal_otlp_bridge_opti_extended::evaluate(fixture).expect("RR-0754: Extended: Journal OTLP bridge optimize registry v4");
    let second = relayring::capabilities::rr_0754_journal_otlp_bridge_opti_extended::evaluate(fixture).expect("RR-0754: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0754: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0754: stats visits every byte");
}
