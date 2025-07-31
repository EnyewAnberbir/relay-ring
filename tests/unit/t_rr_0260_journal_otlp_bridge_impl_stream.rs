//! Integration test for `RR-0260` (stream).
//! Journal OTLP bridge implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0260_journal_otlp_bridge_impl_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let direct = relayring::capabilities::rr_0260_journal_otlp_bridge_impl::evaluate(fixture).expect("RR-0260: direct Journal OTLP bridge implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0260_journal_otlp_bridge_impl::evaluate(&copied).expect("RR-0260: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0260: stream path must consume input");
}
