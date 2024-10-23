//! Integration test for `RR-0196` (roundtrip).
//! Journal append seal extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0196_journal_append_seal_exte_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc7, 0xc9];
    let a = relayring::capabilities::rr_0196_journal_append_seal_exte::evaluate(fixture).expect("RR-0196 first pass");
    let b = relayring::capabilities::rr_0196_journal_append_seal_exte::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
