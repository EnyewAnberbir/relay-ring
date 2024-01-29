//! Integration test for `RR-0261` (bounds).
//! Journal OTLP bridge extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0261_journal_otlp_bridge_exte_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0261_journal_otlp_bridge_exte::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0261_journal_otlp_bridge_exte::evaluate(fixture).expect("RR-0261 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
