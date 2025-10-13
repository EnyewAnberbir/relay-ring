//! Integration test for `RR-0751` (stream).
//! Extended: Journal OTLP bridge extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0751_journal_otlp_bridge_exte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf6, 0xf8];
    let direct = relayring::capabilities::rr_0751_journal_otlp_bridge_exte_extended::evaluate(fixture).expect("RR-0751: direct Extended: Journal OTLP bridge extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0751_journal_otlp_bridge_exte_extended::evaluate(&copied).expect("RR-0751: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0751: stream path must consume input");
}
