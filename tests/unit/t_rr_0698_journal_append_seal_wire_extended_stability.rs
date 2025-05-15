//! Integration test for `RR-0698` (stability).
//! Extended: Journal append seal wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0698_journal_append_seal_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let full = relayring::capabilities::rr_0698_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0698: bulk Extended: Journal append seal wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0698_journal_append_seal_wire_extended::evaluate(&fixture[..end]).expect("RR-0698: stable prefix");
        assert!(partial.consumed <= end, "RR-0698: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0698: full prefix should match bulk checksum");
        }
    }
}
