//! Integration test for `RR-0687` (roundtrip).
//! Extended: Journal append seal harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0687_journal_append_seal_hard_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb6, 0xb8];
    let a = relayring::capabilities::rr_0687_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0687 first pass");
    let b = relayring::capabilities::rr_0687_journal_append_seal_hard_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
