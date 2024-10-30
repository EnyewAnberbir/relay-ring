//! Integration test for `RR-0255` (roundtrip).
//! Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0255_journal_otlp_bridge_vali_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06];
    let a = relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(fixture).expect("RR-0255 first pass");
    let b = relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
