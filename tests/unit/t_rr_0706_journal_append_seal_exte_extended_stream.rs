//! Integration test for `RR-0706` (stream).
//! Extended: Journal append seal extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0706_journal_append_seal_exte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0xcb];
    let direct = relayring::capabilities::rr_0706_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0706: direct Extended: Journal append seal extend codec v31");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0706_journal_append_seal_exte_extended::evaluate(&copied).expect("RR-0706: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0706: stream path must consume input");
}
