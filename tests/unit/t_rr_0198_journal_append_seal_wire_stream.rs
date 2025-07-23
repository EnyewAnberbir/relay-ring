//! Integration test for `RR-0198` (stream).
//! Journal append seal wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0198_journal_append_seal_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0xcb];
    let direct = relayring::capabilities::rr_0198_journal_append_seal_wire::evaluate(fixture).expect("RR-0198: direct Journal append seal wire planner v23");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0198_journal_append_seal_wire::evaluate(&copied).expect("RR-0198: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0198: stream path must consume input");
}
