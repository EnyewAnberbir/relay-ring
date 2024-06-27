//! Integration test for `RR-0366` (empty).
//! Gate surfaces seal index extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0366_gate_surfaces_seal_index_empty() {
    assert!(relayring::capabilities::rr_0366_gate_surfaces_seal_index::evaluate(&[]).is_err(), "RR-0366: empty input must fail for Gate surfaces seal index extend codec v1");
}
