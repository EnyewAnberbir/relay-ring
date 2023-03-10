use super::frame::{JournalRecord, RelayringFrame};

pub fn encode(frame: &RelayringFrame) -> Result<Vec<u8>, String> {
    let mut out = vec![0x52, 0x4c, 0x52, 0x47, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
    out.extend_from_slice(&frame.head_seq.to_le_bytes());
    out.extend_from_slice(&frame.tail_seq.to_le_bytes());
    out.extend_from_slice(&(frame.records.len() as u32).to_le_bytes());
    for rec in &frame.records {
        out.extend_from_slice(&rec.seq.to_le_bytes());
        out.extend_from_slice(&rec.timestamp.to_le_bytes());
        out.extend_from_slice(&(rec.payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&rec.payload);
    }
    Ok(out)
}
