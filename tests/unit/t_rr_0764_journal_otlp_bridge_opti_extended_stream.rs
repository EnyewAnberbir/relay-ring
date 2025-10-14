//! Integration test for `RR-0764` (stream).
//! Extended: Journal OTLP bridge optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0764_journal_otlp_bridge_opti_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x07];
    let direct = relayring::capabilities::rr_0764_journal_otlp_bridge_opti_extended::evaluate(fixture).expect("RR-0764: direct Extended: Journal OTLP bridge optimize registry v14");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0764_journal_otlp_bridge_opti_extended::evaluate(&copied).expect("RR-0764: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0764: stream path must consume input");
}
