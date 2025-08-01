//! Integration test for `RR-0263` (stream).
//! Journal OTLP bridge wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0263_journal_otlp_bridge_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x0e];
    let direct = relayring::capabilities::rr_0263_journal_otlp_bridge_wire::evaluate(fixture).expect("RR-0263: direct Journal OTLP bridge wire planner v13");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0263_journal_otlp_bridge_wire::evaluate(&copied).expect("RR-0263: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0263: stream path must consume input");
}
