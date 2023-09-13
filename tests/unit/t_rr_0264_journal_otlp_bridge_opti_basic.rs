//! Integration test for `RR-0264` (basic).
//! Journal OTLP bridge optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0264_journal_otlp_bridge_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x0f];
    let first = relayring::capabilities::rr_0264_journal_otlp_bridge_opti::evaluate(fixture).expect("RR-0264: Journal OTLP bridge optimize registry v14");
    let second = relayring::capabilities::rr_0264_journal_otlp_bridge_opti::evaluate(fixture).expect("RR-0264: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0264: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0264: stats visits every byte");
}
