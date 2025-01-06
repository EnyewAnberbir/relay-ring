//! Integration test for `RR-0757` (roundtrip).
//! Extended: Journal OTLP bridge integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0757_journal_otlp_bridge_inte_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xfe];
    let a = relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(fixture).expect("RR-0757 first pass");
    let b = relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
