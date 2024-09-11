//! Integration test for `RR-0899` (empty).
//! Extended: Gate compact checksum export optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0899_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0899_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0899: empty input must fail for Extended: Gate compact checksum export optimize registry v4");
}
