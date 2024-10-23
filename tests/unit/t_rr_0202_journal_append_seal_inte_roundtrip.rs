//! Integration test for `RR-0202` (roundtrip).
//! Journal append seal integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0202_journal_append_seal_inte_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let a = relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(fixture).expect("RR-0202 first pass");
    let b = relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
