//! Integration test for `RR-0178` (stream).
//! Journal append seal wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0178_journal_append_seal_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0xb7];
    let direct = relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(fixture).expect("RR-0178: direct Journal append seal wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(&copied).expect("RR-0178: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0178: stream path must consume input");
}
