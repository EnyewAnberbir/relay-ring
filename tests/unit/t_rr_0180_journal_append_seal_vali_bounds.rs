//! Integration test for `RR-0180` (bounds).
//! Journal append seal validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0180_journal_append_seal_vali_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(fixture).expect("RR-0180 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
