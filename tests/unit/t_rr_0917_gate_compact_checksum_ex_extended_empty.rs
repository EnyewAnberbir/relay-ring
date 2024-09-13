//! Integration test for `RR-0917` (empty).
//! Extended: Gate compact checksum export harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0917_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0917_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0917: empty input must fail for Extended: Gate compact checksum export harden index v22");
}
