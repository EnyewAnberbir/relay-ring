//! Integration test for `RR-0251` (stream).
//! Journal OTLP bridge extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0251_journal_otlp_bridge_exte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0x02];
    let direct = relayring::capabilities::rr_0251_journal_otlp_bridge_exte::evaluate(fixture).expect("RR-0251: direct Journal OTLP bridge extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0251_journal_otlp_bridge_exte::evaluate(&copied).expect("RR-0251: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0251: stream path must consume input");
}
