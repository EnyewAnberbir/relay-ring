//! Integration test for `RR-0795` (stream).
//! Extended: Export OTLP batches implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0795_export_otlp_batches_impl_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let direct = relayring::capabilities::rr_0795_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0795: direct Extended: Export OTLP batches implement pipeline v30");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0795_export_otlp_batches_impl_extended::evaluate(&copied).expect("RR-0795: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0795: stream path must consume input");
}
