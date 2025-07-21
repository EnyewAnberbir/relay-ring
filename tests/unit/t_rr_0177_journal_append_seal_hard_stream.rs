//! Integration test for `RR-0177` (stream).
//! Journal append seal harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0177_journal_append_seal_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let direct = relayring::capabilities::rr_0177_journal_append_seal_hard::evaluate(fixture).expect("RR-0177: direct Journal append seal harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0177_journal_append_seal_hard::evaluate(&copied).expect("RR-0177: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0177: stream path must consume input");
}
