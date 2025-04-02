//! Integration test for `RR-0397` (stability).
//! Gate compact checksum export harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0397_gate_compact_checksum_ex_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x92, 0x94];
    let full = relayring::capabilities::rr_0397_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0397: bulk Gate compact checksum export harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0397_gate_compact_checksum_ex::evaluate(&fixture[..end]).expect("RR-0397: stable prefix");
        assert!(partial.consumed <= end, "RR-0397: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0397: full prefix should match bulk checksum");
        }
    }
}
