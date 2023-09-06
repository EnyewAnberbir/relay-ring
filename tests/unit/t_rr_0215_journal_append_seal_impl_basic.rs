//! Integration test for `RR-0215` (basic).
//! Journal append seal implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0215_journal_append_seal_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xda, 0xdc];
    let first = relayring::capabilities::rr_0215_journal_append_seal_impl::evaluate(fixture).expect("RR-0215: Journal append seal implement pipeline v40");
    let second = relayring::capabilities::rr_0215_journal_append_seal_impl::evaluate(fixture).expect("RR-0215: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0215: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0215: stats visits every byte");
}
