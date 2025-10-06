//! Integration test for `RR-0708` (stream).
//! Extended: Journal append seal wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0708_journal_append_seal_wire_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcb, 0xcd];
    let direct = relayring::capabilities::rr_0708_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0708: direct Extended: Journal append seal wire planner v33");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0708_journal_append_seal_wire_extended::evaluate(&copied).expect("RR-0708: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0708: stream path must consume input");
}
