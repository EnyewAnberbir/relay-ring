//! Integration test for `RR-0177` (roundtrip).
//! Journal append seal harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0177_journal_append_seal_hard_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let a = relayring::capabilities::rr_0177_journal_append_seal_hard::evaluate(fixture).expect("RR-0177 first pass");
    let b = relayring::capabilities::rr_0177_journal_append_seal_hard::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
