//! Integration test for `RR-0188` (stream).
//! Journal append seal wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0188_journal_append_seal_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbf, 0xc1];
    let direct = relayring::capabilities::rr_0188_journal_append_seal_wire::evaluate(fixture).expect("RR-0188: direct Journal append seal wire planner v13");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0188_journal_append_seal_wire::evaluate(&copied).expect("RR-0188: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0188: stream path must consume input");
}
