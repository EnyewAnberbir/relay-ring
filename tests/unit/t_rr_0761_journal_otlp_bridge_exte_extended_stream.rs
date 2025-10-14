//! Integration test for `RR-0761` (stream).
//! Extended: Journal OTLP bridge extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0761_journal_otlp_bridge_exte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04];
    let direct = relayring::capabilities::rr_0761_journal_otlp_bridge_exte_extended::evaluate(fixture).expect("RR-0761: direct Extended: Journal OTLP bridge extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0761_journal_otlp_bridge_exte_extended::evaluate(&copied).expect("RR-0761: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0761: stream path must consume input");
}
