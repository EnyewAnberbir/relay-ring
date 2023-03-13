# RelayRing wire format (`.rlrg`)

Magic `RLRG` (4) | version u8 | reserved u8 | record_count u32 LE

Each record: kind u8 | flags u8 | payload_len u16 LE | payload[payload_len] | pad to 4-byte boundary.

Record kinds represent ring segments, journal indexes, and telemetry export batches.

