//! Deployment profile `hot_ring` for relayring.

#[derive(Debug, Clone)]
pub struct HotRingSettings {
    pub ring_segment_bytes: u32,
    pub batch_export_rows: u16,
    pub fsync_every_n: u8,
}

impl HotRingSettings {
    pub fn load() -> Self {
        Self {
            ring_segment_bytes: 1048576,
            batch_export_rows: 512,
            fsync_every_n: 16,
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.ring_segment_bytes == 0 { return Err("ring_segment_bytes"); }
        Ok(())
    }
}
