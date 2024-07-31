//! Integration test for `RR-0591` (empty).
//! Extended: Ring scan lattice modules extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0591_ring_scan_lattice_module_extended_empty() {
    assert!(relayring::capabilities::rr_0591_ring_scan_lattice_module_extended::evaluate(&[]).is_err(), "RR-0591: empty input must fail for Extended: Ring scan lattice modules extend codec v31");
}
