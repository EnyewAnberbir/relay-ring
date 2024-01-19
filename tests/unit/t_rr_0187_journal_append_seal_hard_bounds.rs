//! Integration test for `RR-0187` (bounds).
//! Journal append seal harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0187_journal_append_seal_hard_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0187_journal_append_seal_hard::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0187_journal_append_seal_hard::evaluate(fixture).expect("RR-0187 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
