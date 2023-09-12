//! Integration test for `RR-0260` (basic).
//! Journal OTLP bridge implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0260_journal_otlp_bridge_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let first = relayring::capabilities::rr_0260_journal_otlp_bridge_impl::evaluate(fixture).expect("RR-0260: Journal OTLP bridge implement pipeline v10");
    let second = relayring::capabilities::rr_0260_journal_otlp_bridge_impl::evaluate(fixture).expect("RR-0260: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0260: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0260: stats visits every byte");
}
