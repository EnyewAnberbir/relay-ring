//! Integration test for `RR-0176` (roundtrip).
//! Journal append seal extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0176_journal_append_seal_exte_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb3, 0xb5];
    let a = relayring::capabilities::rr_0176_journal_append_seal_exte::evaluate(fixture).expect("RR-0176 first pass");
    let b = relayring::capabilities::rr_0176_journal_append_seal_exte::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
