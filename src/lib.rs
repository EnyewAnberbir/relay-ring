pub mod util;
pub mod ring { pub mod buffer; pub mod segment; pub mod seqlock_reader; pub mod batch_compress; pub mod batch_align; pub mod segment_lane; pub mod index_cache; pub mod otlp_batch; pub mod relay_agent; pub mod scan_span; pub mod scan_chain; pub mod scan_table; pub mod scan_buffer; pub mod scan_frame; pub mod scan_batch; pub mod scan_slot; pub mod scan_node; pub mod scan_edge; pub mod scan_range; pub mod scan_token; pub mod scan_field; pub mod scan_row; pub mod scan_column; pub mod scan_page; pub mod scan_block; pub mod scan_run; pub mod scan_delta; pub mod scan_cache; pub mod scan_offset; pub mod scan_bucket; }
pub mod journal { pub mod ring_log; pub mod segment_index; pub mod otlp_export; pub mod segment_seal; pub mod sparse_index; pub mod tail_hunter; pub mod compact; }
pub mod export { pub mod batch; pub mod otlp_batch_builder; pub mod retry_jitter; pub mod histogram_agg; pub mod otlp; }
pub mod gateway { pub mod agent; pub mod backpressure_queue; pub mod peer_health; pub mod shard_router; pub mod relay; }
pub mod gates { pub mod agent_ack; pub mod checksum_lane; pub mod compact_pass; pub mod export_batch; pub mod gateway_push; pub mod journal_seek; pub mod offset_index; pub mod replay_scan; pub mod ring_append; pub mod segment_seal; }
pub mod runtime { pub mod sequencer; pub mod shipper; pub mod indexer; pub mod replayer; }
pub mod wire { pub mod frame; pub mod decode; pub mod encode; pub mod validate; }
pub mod telemetry { pub mod journal_report; }
pub mod config { pub mod hot_ring; pub mod cold_archive; pub mod otlp_batch; pub mod agent_edge; pub mod compact_lazy; pub mod fsync_strict; pub mod registry; }

// capabilities subsystem (project extension)
pub mod capabilities;
