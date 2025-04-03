//! Integration test for `RR-0402` (stability).
//! Gate compact checksum export integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0402_gate_compact_checksum_ex_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x99];
    let full = relayring::capabilities::rr_0402_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0402: bulk Gate compact checksum export integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0402_gate_compact_checksum_ex::evaluate(&fixture[..end]).expect("RR-0402: stable prefix");
        assert!(partial.consumed <= end, "RR-0402: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0402: full prefix should match bulk checksum");
        }
    }
}
