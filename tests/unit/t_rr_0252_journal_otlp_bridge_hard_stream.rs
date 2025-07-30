//! Integration test for `RR-0252` (stream).
//! Journal OTLP bridge harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0252_journal_otlp_bridge_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03];
    let direct = relayring::capabilities::rr_0252_journal_otlp_bridge_hard::evaluate(fixture).expect("RR-0252: direct Journal OTLP bridge harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0252_journal_otlp_bridge_hard::evaluate(&copied).expect("RR-0252: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0252: stream path must consume input");
}
