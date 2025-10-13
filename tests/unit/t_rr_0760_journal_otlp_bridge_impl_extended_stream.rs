//! Integration test for `RR-0760` (stream).
//! Extended: Journal OTLP bridge implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0760_journal_otlp_bridge_impl_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03];
    let direct = relayring::capabilities::rr_0760_journal_otlp_bridge_impl_extended::evaluate(fixture).expect("RR-0760: direct Extended: Journal OTLP bridge implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0760_journal_otlp_bridge_impl_extended::evaluate(&copied).expect("RR-0760: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0760: stream path must consume input");
}
