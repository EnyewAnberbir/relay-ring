//! Integration test for `RR-0706` (roundtrip).
//! Extended: Journal append seal extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0706_journal_append_seal_exte_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0xcb];
    let a = relayring::capabilities::rr_0706_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0706 first pass");
    let b = relayring::capabilities::rr_0706_journal_append_seal_exte_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
