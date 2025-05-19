//! Integration test for `RR-0708` (stability).
//! Extended: Journal append seal wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0708_journal_append_seal_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcb, 0xcd];
    let full = relayring::capabilities::rr_0708_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0708: bulk Extended: Journal append seal wire planner v33");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0708_journal_append_seal_wire_extended::evaluate(&fixture[..end]).expect("RR-0708: stable prefix");
        assert!(partial.consumed <= end, "RR-0708: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0708: full prefix should match bulk checksum");
        }
    }
}
