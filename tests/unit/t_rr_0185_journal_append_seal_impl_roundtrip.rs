//! Integration test for `RR-0185` (roundtrip).
//! Journal append seal implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0185_journal_append_seal_impl_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let a = relayring::capabilities::rr_0185_journal_append_seal_impl::evaluate(fixture).expect("RR-0185 first pass");
    let b = relayring::capabilities::rr_0185_journal_append_seal_impl::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
