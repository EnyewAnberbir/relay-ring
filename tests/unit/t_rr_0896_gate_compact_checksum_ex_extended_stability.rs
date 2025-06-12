//! Integration test for `RR-0896` (stability).
//! Extended: Gate compact checksum export extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0896_gate_compact_checksum_ex_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x89, 0x8b];
    let full = relayring::capabilities::rr_0896_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0896: bulk Extended: Gate compact checksum export extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0896_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]).expect("RR-0896: stable prefix");
        assert!(partial.consumed <= end, "RR-0896: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0896: full prefix should match bulk checksum");
        }
    }
}
