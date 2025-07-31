//! Integration test for `RR-0253` (stream).
//! Journal OTLP bridge wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0253_journal_otlp_bridge_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04];
    let direct = relayring::capabilities::rr_0253_journal_otlp_bridge_wire::evaluate(fixture).expect("RR-0253: direct Journal OTLP bridge wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0253_journal_otlp_bridge_wire::evaluate(&copied).expect("RR-0253: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0253: stream path must consume input");
}
