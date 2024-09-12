//! Integration test for `RR-0906` (empty).
//! Extended: Gate compact checksum export extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0906_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0906_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0906: empty input must fail for Extended: Gate compact checksum export extend codec v11");
}
