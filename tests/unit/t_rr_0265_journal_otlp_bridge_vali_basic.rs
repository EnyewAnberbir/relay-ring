//! Integration test for `RR-0265` (basic).
//! Journal OTLP bridge validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0265_journal_otlp_bridge_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let first = relayring::capabilities::rr_0265_journal_otlp_bridge_vali::evaluate(fixture).expect("RR-0265: Journal OTLP bridge validate resolver v15");
    let second = relayring::capabilities::rr_0265_journal_otlp_bridge_vali::evaluate(fixture).expect("RR-0265: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0265: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0265: window consumes the whole buffer");
}
