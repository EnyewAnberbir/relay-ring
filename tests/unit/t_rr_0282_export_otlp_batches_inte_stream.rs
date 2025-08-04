//! Integration test for `RR-0282` (stream).
//! Export OTLP batches integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0282_export_otlp_batches_inte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let direct = relayring::capabilities::rr_0282_export_otlp_batches_inte::evaluate(fixture).expect("RR-0282: direct Export OTLP batches integrate validator v17");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0282_export_otlp_batches_inte::evaluate(&copied).expect("RR-0282: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0282: stream path must consume input");
}
