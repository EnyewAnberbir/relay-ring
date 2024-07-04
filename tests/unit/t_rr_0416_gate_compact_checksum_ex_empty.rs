//! Integration test for `RR-0416` (empty).
//! Gate compact checksum export extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0416_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0416_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0416: empty input must fail for Gate compact checksum export extend codec v21");
}
