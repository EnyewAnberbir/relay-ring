//! Integration test for `RR-0265` (stability).
//! Journal OTLP bridge validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0265_journal_otlp_bridge_vali_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let full = relayring::capabilities::rr_0265_journal_otlp_bridge_vali::evaluate(fixture).expect("RR-0265: bulk Journal OTLP bridge validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0265_journal_otlp_bridge_vali::evaluate(&fixture[..end]).expect("RR-0265: stable prefix");
        assert!(partial.consumed <= end, "RR-0265: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0265: full prefix should match bulk checksum");
        }
    }
}
