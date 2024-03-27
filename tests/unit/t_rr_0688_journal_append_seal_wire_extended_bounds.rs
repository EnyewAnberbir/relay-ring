//! Integration test for `RR-0688` (bounds).
//! Extended: Journal append seal wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0688_journal_append_seal_wire_extended_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0688_journal_append_seal_wire_extended::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0688_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0688 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
