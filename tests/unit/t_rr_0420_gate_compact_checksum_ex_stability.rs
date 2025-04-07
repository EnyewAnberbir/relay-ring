//! Integration test for `RR-0420` (stability).
//! Gate compact checksum export validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0420_gate_compact_checksum_ex_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa9, 0xab];
    let full = relayring::capabilities::rr_0420_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0420: bulk Gate compact checksum export validate resolver v25");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0420_gate_compact_checksum_ex::evaluate(&fixture[..end]).expect("RR-0420: stable prefix");
        assert!(partial.consumed <= end, "RR-0420: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0420: full prefix should match bulk checksum");
        }
    }
}
