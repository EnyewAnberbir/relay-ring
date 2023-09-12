//! Integration test for `RR-0257` (basic).
//! Journal OTLP bridge integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0257_journal_otlp_bridge_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x08];
    let first = relayring::capabilities::rr_0257_journal_otlp_bridge_inte::evaluate(fixture).expect("RR-0257: Journal OTLP bridge integrate validator v7");
    let second = relayring::capabilities::rr_0257_journal_otlp_bridge_inte::evaluate(fixture).expect("RR-0257: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0257: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0257: stats visits every byte");
}
