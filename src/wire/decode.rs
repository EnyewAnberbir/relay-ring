use super::frame::{JournalRecord, RelayringFrame};

pub fn decode(data: &[u8]) -> Result<RelayringFrame, String> {
    if data.len() < 24 || &data[..4] != [0x52, 0x4c, 0x52, 0x47] { return Err("rlrg journal".into()); }
    let head_seq = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let tail_seq = u64::from_le_bytes(data[16..24].try_into().unwrap());
    let count = u32::from_le_bytes(data[24..28].try_into().unwrap()) as usize;
    let mut cursor = 28usize;
    let mut records = Vec::new();
    for _ in 0..count.min(8192) {
        if cursor + 20 > data.len() { break; }
        let seq = u64::from_le_bytes(data[cursor..cursor + 8].try_into().unwrap());
        let ts = u64::from_le_bytes(data[cursor + 8..cursor + 16].try_into().unwrap());
        let len = u32::from_le_bytes(data[cursor + 16..cursor + 20].try_into().unwrap()) as usize;
        cursor += 20;
        if cursor + len > data.len() { break; }
        let payload = data[cursor..cursor + len].to_vec();
        cursor += len;
        records.push(JournalRecord { seq, timestamp: ts, payload });
    }
    Ok(RelayringFrame { head_seq, tail_seq, records })
}

pub fn synthetic_from_payload(raw: &[u8]) -> RelayringFrame {
    RelayringFrame { head_seq: 0, tail_seq: 1, records: vec![JournalRecord { seq: 0, timestamp: 0, payload: raw.to_vec() }] }
}
