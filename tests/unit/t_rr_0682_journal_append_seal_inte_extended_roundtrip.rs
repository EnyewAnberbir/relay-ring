//! Integration test for `RR-0682` (roundtrip).
//! Extended: Journal append seal integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0682_journal_append_seal_inte_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let a = relayring::capabilities::rr_0682_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0682 first pass");
    let b = relayring::capabilities::rr_0682_journal_append_seal_inte_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
