//! Integration test for `RR-0184` (roundtrip).
//! Journal append seal benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0184_journal_append_seal_benc_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let a = relayring::capabilities::rr_0184_journal_append_seal_benc::evaluate(fixture).expect("RR-0184 first pass");
    let b = relayring::capabilities::rr_0184_journal_append_seal_benc::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
