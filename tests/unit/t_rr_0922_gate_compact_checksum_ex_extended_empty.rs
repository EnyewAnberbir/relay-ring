//! Integration test for `RR-0922` (empty).
//! Extended: Gate compact checksum export integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0922_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0922_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0922: empty input must fail for Extended: Gate compact checksum export integrate validator v27");
}
