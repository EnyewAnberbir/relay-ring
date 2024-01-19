//! Integration test for `RR-0185` (bounds).
//! Journal append seal implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0185_journal_append_seal_impl_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0185_journal_append_seal_impl::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0185_journal_append_seal_impl::evaluate(fixture).expect("RR-0185 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
