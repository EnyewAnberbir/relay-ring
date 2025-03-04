//! Integration test for `RR-0178` (stability).
//! Journal append seal wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0178_journal_append_seal_wire_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0xb7];
    let full = relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(fixture).expect("RR-0178: bulk Journal append seal wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(&fixture[..end]).expect("RR-0178: stable prefix");
        assert!(partial.consumed <= end, "RR-0178: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0178: full prefix should match bulk checksum");
        }
    }
}
