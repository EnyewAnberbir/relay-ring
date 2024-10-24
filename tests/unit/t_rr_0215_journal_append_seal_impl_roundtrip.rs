//! Integration test for `RR-0215` (roundtrip).
//! Journal append seal implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0215_journal_append_seal_impl_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xda, 0xdc];
    let a = relayring::capabilities::rr_0215_journal_append_seal_impl::evaluate(fixture).expect("RR-0215 first pass");
    let b = relayring::capabilities::rr_0215_journal_append_seal_impl::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
