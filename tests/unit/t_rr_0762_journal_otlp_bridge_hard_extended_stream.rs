//! Integration test for `RR-0762` (stream).
//! Extended: Journal OTLP bridge harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0762_journal_otlp_bridge_hard_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x05];
    let direct = relayring::capabilities::rr_0762_journal_otlp_bridge_hard_extended::evaluate(fixture).expect("RR-0762: direct Extended: Journal OTLP bridge harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0762_journal_otlp_bridge_hard_extended::evaluate(&copied).expect("RR-0762: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0762: stream path must consume input");
}
