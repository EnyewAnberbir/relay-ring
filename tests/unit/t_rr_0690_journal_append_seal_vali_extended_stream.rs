//! Integration test for `RR-0690` (stream).
//! Extended: Journal append seal validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0690_journal_append_seal_vali_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb9, 0xbb];
    let direct = relayring::capabilities::rr_0690_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0690: direct Extended: Journal append seal validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0690_journal_append_seal_vali_extended::evaluate(&copied).expect("RR-0690: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0690: stream path must consume input");
}
