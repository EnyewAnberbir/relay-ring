//! Integration test for `RR-0254` (stream).
//! Journal OTLP bridge optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0254_journal_otlp_bridge_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x05];
    let direct = relayring::capabilities::rr_0254_journal_otlp_bridge_opti::evaluate(fixture).expect("RR-0254: direct Journal OTLP bridge optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0254_journal_otlp_bridge_opti::evaluate(&copied).expect("RR-0254: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0254: stream path must consume input");
}
