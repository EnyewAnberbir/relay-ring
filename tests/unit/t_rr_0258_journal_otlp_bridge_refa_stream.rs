//! Integration test for `RR-0258` (stream).
//! Journal OTLP bridge refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0258_journal_otlp_bridge_refa_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x09];
    let direct = relayring::capabilities::rr_0258_journal_otlp_bridge_refa::evaluate(fixture).expect("RR-0258: direct Journal OTLP bridge refactor mutator v8");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0258_journal_otlp_bridge_refa::evaluate(&copied).expect("RR-0258: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0258: stream path must consume input");
}
