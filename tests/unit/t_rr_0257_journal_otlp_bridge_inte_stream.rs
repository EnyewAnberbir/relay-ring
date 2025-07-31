//! Integration test for `RR-0257` (stream).
//! Journal OTLP bridge integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0257_journal_otlp_bridge_inte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x08];
    let direct = relayring::capabilities::rr_0257_journal_otlp_bridge_inte::evaluate(fixture).expect("RR-0257: direct Journal OTLP bridge integrate validator v7");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0257_journal_otlp_bridge_inte::evaluate(&copied).expect("RR-0257: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0257: stream path must consume input");
}
