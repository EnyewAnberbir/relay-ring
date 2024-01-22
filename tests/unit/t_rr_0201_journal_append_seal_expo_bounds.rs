//! Integration test for `RR-0201` (bounds).
//! Journal append seal export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0201_journal_append_seal_expo_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(fixture).expect("RR-0201 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
