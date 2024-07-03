//! Integration test for `RR-0409` (empty).
//! Gate compact checksum export optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0409_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0409_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0409: empty input must fail for Gate compact checksum export optimize registry v14");
}
