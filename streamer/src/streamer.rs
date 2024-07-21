impl StreamerReceiveStats {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            packets_count: AtomicUsize::default(),
            packet_batches_count: AtomicUsize::default(),
            full_packet_batches_count: AtomicUsize::default(),
            max_channel_len: AtomicUsize::default(),
        }
    }

    pub fn report(&self) {
        datapoint_info!(
            self.name,
            (
                "packets_count",
                self.packets_count.swap(0, Ordering::Relaxed) as i64,
                i64
            ),
            (
                "packet_batches_count",
                self.packet_batches_count.swap(0, Ordering::Relaxed) as i64,
                i64
            ),
            (
                "full_packet_batches_count",
                self.full_packet_batches_count.swap(0, Ordering::Relaxed) as i64,
                i64
            ),
            (
                "channel_len",
                self.max_channel_len.swap(0, Ordering::Relaxed) as i64,
                i64
            ),
        );
    }
}
