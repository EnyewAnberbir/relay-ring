//! Integration test for `RR-0259` (roundtrip).
//! Journal OTLP bridge benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0259_journal_otlp_bridge_benc_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let a = relayring::capabilities::rr_0259_journal_otlp_bridge_benc::evaluate(fixture).expect("RR-0259 first pass");
    let b = relayring::capabilities::rr_0259_journal_otlp_bridge_benc::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
