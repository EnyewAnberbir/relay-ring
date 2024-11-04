//! Integration test for `RR-0282` (roundtrip).
//! Export OTLP batches integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0282_export_otlp_batches_inte_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let a = relayring::capabilities::rr_0282_export_otlp_batches_inte::evaluate(fixture).expect("RR-0282 first pass");
    let b = relayring::capabilities::rr_0282_export_otlp_batches_inte::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
