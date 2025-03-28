//! Integration test for `RR-0366` (stability).
//! Gate surfaces seal index extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0366_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x73, 0x75];
    let full = relayring::capabilities::rr_0366_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0366: bulk Gate surfaces seal index extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0366_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0366: stable prefix");
        assert!(partial.consumed <= end, "RR-0366: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0366: full prefix should match bulk checksum");
        }
    }
}
