//! Integration test for `RR-0922` (stability).
//! Extended: Gate compact checksum export integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0922_gate_compact_checksum_ex_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xa5];
    let full = relayring::capabilities::rr_0922_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0922: bulk Extended: Gate compact checksum export integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0922_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]).expect("RR-0922: stable prefix");
        assert!(partial.consumed <= end, "RR-0922: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0922: full prefix should match bulk checksum");
        }
    }
}
