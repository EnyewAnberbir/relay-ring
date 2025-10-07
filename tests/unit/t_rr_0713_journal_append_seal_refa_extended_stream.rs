//! Integration test for `RR-0713` (stream).
//! Extended: Journal append seal refactor mutator v38 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0713_journal_append_seal_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0xd2];
    let direct = relayring::capabilities::rr_0713_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0713: direct Extended: Journal append seal refactor mutator v38");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0713_journal_append_seal_refa_extended::evaluate(&copied).expect("RR-0713: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0713: stream path must consume input");
}
