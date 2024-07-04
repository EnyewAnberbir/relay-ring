//! Integration test for `RR-0411` (empty).
//! Gate compact checksum export export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0411_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0411_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0411: empty input must fail for Gate compact checksum export export adapter v16");
}
