//! Integration test for `RR-0261` (stream).
//! Journal OTLP bridge extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0261_journal_otlp_bridge_exte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x0c];
    let direct = relayring::capabilities::rr_0261_journal_otlp_bridge_exte::evaluate(fixture).expect("RR-0261: direct Journal OTLP bridge extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0261_journal_otlp_bridge_exte::evaluate(&copied).expect("RR-0261: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0261: stream path must consume input");
}
