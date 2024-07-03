//! Integration test for `RR-0402` (empty).
//! Gate compact checksum export integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0402_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0402_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0402: empty input must fail for Gate compact checksum export integrate validator v7");
}
