//! Integration test for `RR-0255` (stability).
//! Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0255_journal_otlp_bridge_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06];
    let full = relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(fixture).expect("RR-0255: bulk Journal OTLP bridge validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(&fixture[..end]).expect("RR-0255: stable prefix");
        assert!(partial.consumed <= end, "RR-0255: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0255: full prefix should match bulk checksum");
        }
    }
}
