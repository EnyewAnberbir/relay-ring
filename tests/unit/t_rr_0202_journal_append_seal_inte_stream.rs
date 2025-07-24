//! Integration test for `RR-0202` (stream).
//! Journal append seal integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0202_journal_append_seal_inte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let direct = relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(fixture).expect("RR-0202: direct Journal append seal integrate validator v27");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(&copied).expect("RR-0202: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0202: stream path must consume input");
}
