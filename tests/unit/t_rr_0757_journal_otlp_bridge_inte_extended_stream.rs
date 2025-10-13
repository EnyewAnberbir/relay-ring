//! Integration test for `RR-0757` (stream).
//! Extended: Journal OTLP bridge integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0757_journal_otlp_bridge_inte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xfe];
    let direct = relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(fixture).expect("RR-0757: direct Extended: Journal OTLP bridge integrate validator v7");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(&copied).expect("RR-0757: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0757: stream path must consume input");
}
