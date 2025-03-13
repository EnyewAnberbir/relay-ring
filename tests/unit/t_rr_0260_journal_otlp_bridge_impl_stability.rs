//! Integration test for `RR-0260` (stability).
//! Journal OTLP bridge implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0260_journal_otlp_bridge_impl_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let full = relayring::capabilities::rr_0260_journal_otlp_bridge_impl::evaluate(fixture).expect("RR-0260: bulk Journal OTLP bridge implement pipeline v10");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0260_journal_otlp_bridge_impl::evaluate(&fixture[..end]).expect("RR-0260: stable prefix");
        assert!(partial.consumed <= end, "RR-0260: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0260: full prefix should match bulk checksum");
        }
    }
}
