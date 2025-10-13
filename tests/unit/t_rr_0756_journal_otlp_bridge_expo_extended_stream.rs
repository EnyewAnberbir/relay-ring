//! Integration test for `RR-0756` (stream).
//! Extended: Journal OTLP bridge export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0756_journal_otlp_bridge_expo_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0xfd];
    let direct = relayring::capabilities::rr_0756_journal_otlp_bridge_expo_extended::evaluate(fixture).expect("RR-0756: direct Extended: Journal OTLP bridge export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0756_journal_otlp_bridge_expo_extended::evaluate(&copied).expect("RR-0756: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0756: stream path must consume input");
}
