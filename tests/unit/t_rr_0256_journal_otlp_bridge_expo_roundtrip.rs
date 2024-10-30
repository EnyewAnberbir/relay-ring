//! Integration test for `RR-0256` (roundtrip).
//! Journal OTLP bridge export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0256_journal_otlp_bridge_expo_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x07];
    let a = relayring::capabilities::rr_0256_journal_otlp_bridge_expo::evaluate(fixture).expect("RR-0256 first pass");
    let b = relayring::capabilities::rr_0256_journal_otlp_bridge_expo::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
