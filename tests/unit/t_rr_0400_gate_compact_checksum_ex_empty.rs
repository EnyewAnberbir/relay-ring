//! Integration test for `RR-0400` (empty).
//! Gate compact checksum export validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0400_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0400_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0400: empty input must fail for Gate compact checksum export validate resolver v5");
}
