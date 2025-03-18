//! Integration test for `RR-0282` (stability).
//! Export OTLP batches integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0282_export_otlp_batches_inte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let full = relayring::capabilities::rr_0282_export_otlp_batches_inte::evaluate(fixture).expect("RR-0282: bulk Export OTLP batches integrate validator v17");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0282_export_otlp_batches_inte::evaluate(&fixture[..end]).expect("RR-0282: stable prefix");
        assert!(partial.consumed <= end, "RR-0282: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0282: full prefix should match bulk checksum");
        }
    }
}
