//! Deployment profile `otlp_batch` for relayring.

#[derive(Debug, Clone)]
pub struct OtlpBatchSettings {
    pub ring_segment_bytes: u32,
    pub batch_export_rows: u16,
    pub fsync_every_n: u8,
}

impl OtlpBatchSettings {
    pub fn load() -> Self {
        Self {
            ring_segment_bytes: 2097152,
            batch_export_rows: 1024,
            fsync_every_n: 32,
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.ring_segment_bytes == 0 { return Err("ring_segment_bytes"); }
        Ok(())
    }
}
