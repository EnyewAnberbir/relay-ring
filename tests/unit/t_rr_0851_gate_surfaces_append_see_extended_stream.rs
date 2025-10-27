//! Integration test for `RR-0851` (stream).
//! Extended: Gate surfaces append seek export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0851_gate_surfaces_append_see_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5c, 0x5e];
    let direct = relayring::capabilities::rr_0851_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0851: direct Extended: Gate surfaces append seek export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0851_gate_surfaces_append_see_extended::evaluate(&copied).expect("RR-0851: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0851: stream path must consume input");
}
