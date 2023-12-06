//! Integration test for `RR-0889` (basic).
//! Extended: Gate surfaces seal index optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0889_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x82, 0x84];
    let first = relayring::capabilities::rr_0889_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0889: Extended: Gate surfaces seal index optimize registry v24");
    let second = relayring::capabilities::rr_0889_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0889: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0889: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0889: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
