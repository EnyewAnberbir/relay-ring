//! Integration test for `RR-0212` (stream).
//! Journal append seal integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0212_journal_append_seal_inte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd7, 0xd9];
    let direct = relayring::capabilities::rr_0212_journal_append_seal_inte::evaluate(fixture).expect("RR-0212: direct Journal append seal integrate validator v37");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0212_journal_append_seal_inte::evaluate(&copied).expect("RR-0212: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0212: stream path must consume input");
}
