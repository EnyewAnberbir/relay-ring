//! Integration test for `RR-0871` (empty).
//! Extended: Gate surfaces seal index export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0871_gate_surfaces_seal_index_extended_empty() {
    assert!(relayring::capabilities::rr_0871_gate_surfaces_seal_index_extended::evaluate(&[]).is_err(), "RR-0871: empty input must fail for Extended: Gate surfaces seal index export adapter v6");
}
