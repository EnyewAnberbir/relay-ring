//! Integration test for `RR-0191` (roundtrip).
//! Journal append seal export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0191_journal_append_seal_expo_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xc4];
    let a = relayring::capabilities::rr_0191_journal_append_seal_expo::evaluate(fixture).expect("RR-0191 first pass");
    let b = relayring::capabilities::rr_0191_journal_append_seal_expo::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
