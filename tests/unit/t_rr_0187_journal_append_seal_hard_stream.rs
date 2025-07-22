//! Integration test for `RR-0187` (stream).
//! Journal append seal harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0187_journal_append_seal_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbe, 0xc0];
    let direct = relayring::capabilities::rr_0187_journal_append_seal_hard::evaluate(fixture).expect("RR-0187: direct Journal append seal harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0187_journal_append_seal_hard::evaluate(&copied).expect("RR-0187: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0187: stream path must consume input");
}
