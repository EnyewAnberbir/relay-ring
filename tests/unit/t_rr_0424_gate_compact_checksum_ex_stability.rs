//! Integration test for `RR-0424` (stability).
//! Gate compact checksum export benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0424_gate_compact_checksum_ex_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xad, 0xaf];
    let full = relayring::capabilities::rr_0424_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0424: bulk Gate compact checksum export benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0424_gate_compact_checksum_ex::evaluate(&fixture[..end]).expect("RR-0424: stable prefix");
        assert!(partial.consumed <= end, "RR-0424: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0424: full prefix should match bulk checksum");
        }
    }
}
