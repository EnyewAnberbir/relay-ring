pub struct ProfileRegistry {
    pub active: &'static str,
    pub available: &'static [&'static str],
}

impl ProfileRegistry {
    pub fn bootstrap() -> Self {
        Self {
            active: "hot_ring",
            available: &["hot_ring", "cold_archive", "otlp_batch", "agent_edge", "compact_lazy", "fsync_strict"],
        }
    }

    pub fn set_active(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(&profile) = self.available.iter().find(|&&p| p == name) {
            self.active = profile;
            Ok(())
        } else {
            Err("unknown profile")
        }
    }

    pub fn active_name(&self) -> &'static str {
        self.active
    }
}
