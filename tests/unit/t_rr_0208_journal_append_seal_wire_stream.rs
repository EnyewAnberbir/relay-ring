//! Integration test for `RR-0208` (stream).
//! Journal append seal wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0208_journal_append_seal_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd3, 0xd5];
    let direct = relayring::capabilities::rr_0208_journal_append_seal_wire::evaluate(fixture).expect("RR-0208: direct Journal append seal wire planner v33");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0208_journal_append_seal_wire::evaluate(&copied).expect("RR-0208: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0208: stream path must consume input");
}
