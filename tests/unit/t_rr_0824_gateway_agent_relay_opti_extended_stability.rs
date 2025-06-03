//! Integration test for `RR-0824` (stability).
//! Extended: Gateway agent relay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0824_gateway_agent_relay_opti_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x43];
    let full = relayring::capabilities::rr_0824_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0824: bulk Extended: Gateway agent relay optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0824_gateway_agent_relay_opti_extended::evaluate(&fixture[..end]).expect("RR-0824: stable prefix");
        assert!(partial.consumed <= end, "RR-0824: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0824: full prefix should match bulk checksum");
        }
    }
}
