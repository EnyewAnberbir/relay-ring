//! Integration test for `RR-0762` (roundtrip).
//! Extended: Journal OTLP bridge harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0762_journal_otlp_bridge_hard_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x05];
    let a = relayring::capabilities::rr_0762_journal_otlp_bridge_hard_extended::evaluate(fixture).expect("RR-0762 first pass");
    let b = relayring::capabilities::rr_0762_journal_otlp_bridge_hard_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
