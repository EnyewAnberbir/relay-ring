//! Integration test for `RR-0197` (stream).
//! Journal append seal harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0197_journal_append_seal_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let direct = relayring::capabilities::rr_0197_journal_append_seal_hard::evaluate(fixture).expect("RR-0197: direct Journal append seal harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0197_journal_append_seal_hard::evaluate(&copied).expect("RR-0197: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0197: stream path must consume input");
}
