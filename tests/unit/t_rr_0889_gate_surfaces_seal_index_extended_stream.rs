//! Integration test for `RR-0889` (stream).
//! Extended: Gate surfaces seal index optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0889_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x82, 0x84];
    let direct = relayring::capabilities::rr_0889_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0889: direct Extended: Gate surfaces seal index optimize registry v24");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0889_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0889: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0889: stream path must consume input");
}
