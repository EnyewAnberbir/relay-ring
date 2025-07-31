//! Integration test for `RR-0256` (stream).
//! Journal OTLP bridge export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0256_journal_otlp_bridge_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x07];
    let direct = relayring::capabilities::rr_0256_journal_otlp_bridge_expo::evaluate(fixture).expect("RR-0256: direct Journal OTLP bridge export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0256_journal_otlp_bridge_expo::evaluate(&copied).expect("RR-0256: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0256: stream path must consume input");
}
