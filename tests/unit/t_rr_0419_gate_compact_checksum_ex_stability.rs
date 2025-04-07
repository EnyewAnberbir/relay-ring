//! Integration test for `RR-0419` (stability).
//! Gate compact checksum export optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0419_gate_compact_checksum_ex_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0xaa];
    let full = relayring::capabilities::rr_0419_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0419: bulk Gate compact checksum export optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0419_gate_compact_checksum_ex::evaluate(&fixture[..end]).expect("RR-0419: stable prefix");
        assert!(partial.consumed <= end, "RR-0419: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0419: full prefix should match bulk checksum");
        }
    }
}
