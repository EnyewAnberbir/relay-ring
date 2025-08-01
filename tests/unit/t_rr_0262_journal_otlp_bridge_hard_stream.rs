//! Integration test for `RR-0262` (stream).
//! Journal OTLP bridge harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0262_journal_otlp_bridge_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x0d];
    let direct = relayring::capabilities::rr_0262_journal_otlp_bridge_hard::evaluate(fixture).expect("RR-0262: direct Journal OTLP bridge harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0262_journal_otlp_bridge_hard::evaluate(&copied).expect("RR-0262: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0262: stream path must consume input");
}
