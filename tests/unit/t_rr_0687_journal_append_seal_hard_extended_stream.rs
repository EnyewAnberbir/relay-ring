//! Integration test for `RR-0687` (stream).
//! Extended: Journal append seal harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0687_journal_append_seal_hard_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb6, 0xb8];
    let direct = relayring::capabilities::rr_0687_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0687: direct Extended: Journal append seal harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0687_journal_append_seal_hard_extended::evaluate(&copied).expect("RR-0687: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0687: stream path must consume input");
}
