//! Integration test for `RR-0759` (stream).
//! Extended: Journal OTLP bridge benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0759_journal_otlp_bridge_benc_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0x02];
    let direct = relayring::capabilities::rr_0759_journal_otlp_bridge_benc_extended::evaluate(fixture).expect("RR-0759: direct Extended: Journal OTLP bridge benchmark reporter v9");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0759_journal_otlp_bridge_benc_extended::evaluate(&copied).expect("RR-0759: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0759: stream path must consume input");
}
