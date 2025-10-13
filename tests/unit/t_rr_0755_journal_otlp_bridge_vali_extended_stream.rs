//! Integration test for `RR-0755` (stream).
//! Extended: Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0755_journal_otlp_bridge_vali_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0xfc];
    let direct = relayring::capabilities::rr_0755_journal_otlp_bridge_vali_extended::evaluate(fixture).expect("RR-0755: direct Extended: Journal OTLP bridge validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0755_journal_otlp_bridge_vali_extended::evaluate(&copied).expect("RR-0755: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0755: stream path must consume input");
}
