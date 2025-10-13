//! Integration test for `RR-0754` (stream).
//! Extended: Journal OTLP bridge optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0754_journal_otlp_bridge_opti_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0xfb];
    let direct = relayring::capabilities::rr_0754_journal_otlp_bridge_opti_extended::evaluate(fixture).expect("RR-0754: direct Extended: Journal OTLP bridge optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0754_journal_otlp_bridge_opti_extended::evaluate(&copied).expect("RR-0754: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0754: stream path must consume input");
}
