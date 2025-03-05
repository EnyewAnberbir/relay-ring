//! Integration test for `RR-0188` (stability).
//! Journal append seal wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0188_journal_append_seal_wire_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbf, 0xc1];
    let full = relayring::capabilities::rr_0188_journal_append_seal_wire::evaluate(fixture).expect("RR-0188: bulk Journal append seal wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0188_journal_append_seal_wire::evaluate(&fixture[..end]).expect("RR-0188: stable prefix");
        assert!(partial.consumed <= end, "RR-0188: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0188: full prefix should match bulk checksum");
        }
    }
}
