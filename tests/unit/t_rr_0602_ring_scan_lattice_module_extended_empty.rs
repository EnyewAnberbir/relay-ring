//! Integration test for `RR-0602` (empty).
//! Extended: Ring scan lattice modules harden index v42 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0602_ring_scan_lattice_module_extended_empty() {
    assert!(relayring::capabilities::rr_0602_ring_scan_lattice_module_extended::evaluate(&[]).is_err(), "RR-0602: empty input must fail for Extended: Ring scan lattice modules harden index v42");
}
