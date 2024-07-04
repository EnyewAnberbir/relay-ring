//! Integration test for `RR-0420` (empty).
//! Gate compact checksum export validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0420_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0420_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0420: empty input must fail for Gate compact checksum export validate resolver v25");
}
