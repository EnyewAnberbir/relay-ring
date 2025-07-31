//! Integration test for `RR-0255` (stream).
//! Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0255_journal_otlp_bridge_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06];
    let direct = relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(fixture).expect("RR-0255: direct Journal OTLP bridge validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(&copied).expect("RR-0255: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0255: stream path must consume input");
}
