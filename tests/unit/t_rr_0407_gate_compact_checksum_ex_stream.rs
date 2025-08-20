//! Integration test for `RR-0407` (stream).
//! Gate compact checksum export harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0407_gate_compact_checksum_ex_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9c, 0x9e];
    let direct = relayring::capabilities::rr_0407_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0407: direct Gate compact checksum export harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0407_gate_compact_checksum_ex::evaluate(&copied).expect("RR-0407: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0407: stream path must consume input");
}
