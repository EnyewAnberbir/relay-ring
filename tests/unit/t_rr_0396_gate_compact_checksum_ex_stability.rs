//! Integration test for `RR-0396` (stability).
//! Gate compact checksum export extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0396_gate_compact_checksum_ex_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x93];
    let full = relayring::capabilities::rr_0396_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0396: bulk Gate compact checksum export extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0396_gate_compact_checksum_ex::evaluate(&fixture[..end]).expect("RR-0396: stable prefix");
        assert!(partial.consumed <= end, "RR-0396: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0396: full prefix should match bulk checksum");
        }
    }
}
