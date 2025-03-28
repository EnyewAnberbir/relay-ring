//! Integration test for `RR-0362` (stability).
//! Gate surfaces append seek integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0362_gate_surfaces_append_see_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71];
    let full = relayring::capabilities::rr_0362_gate_surfaces_append_see::evaluate(fixture).expect("RR-0362: bulk Gate surfaces append seek integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0362_gate_surfaces_append_see::evaluate(&fixture[..end]).expect("RR-0362: stable prefix");
        assert!(partial.consumed <= end, "RR-0362: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0362: full prefix should match bulk checksum");
        }
    }
}
