//! Integration test for `RR-0179` (roundtrip).
//! Journal append seal optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0179_journal_append_seal_opti_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb6, 0xb8];
    let a = relayring::capabilities::rr_0179_journal_append_seal_opti::evaluate(fixture).expect("RR-0179 first pass");
    let b = relayring::capabilities::rr_0179_journal_append_seal_opti::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
