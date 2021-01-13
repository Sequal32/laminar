/// Metrics to be sent every second 
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Metrics {
    /// Packets sent
    pub sent_packets: usize,
    /// Packets received
    pub received_packets: usize,
    /// The amount of data in kilobytes sent
    pub sent_kbps: f32,
    /// The amount of data in kilobytes received
    pub receive_kbps: f32,
    /// The percentage (0-1) packets lost
    pub packet_loss: f32,
    /// Round trip time
    pub rtt: f32
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            sent_packets: 0,
            received_packets: 0,
            sent_kbps: 0.0,
            receive_kbps: 0.0,
            packet_loss: 0.0,
            rtt: 0.0,
        }
    }
}

/// A "frame" of metrics, where all the values in Vecs will be averaged into a Metrics struct
struct MetricsArray {
    pub sent_kbps: Vec<f32>,
    pub receive_kbps: Vec<f32>,
    pub packet_loss: usize,
    pub rtt: Vec<f32>
}

impl Default for MetricsArray {
    fn default() -> Self {
        Self {
            sent_kbps: Vec::new(),
            receive_kbps: Vec::new(),
            packet_loss: 0,
            rtt: Vec::new(),
        }
    }
}

impl MetricsArray {
    pub fn reset(&mut self) {
        self.sent_kbps.clear();
        self.receive_kbps.clear();
        self.rtt.clear();
        self.packet_loss = 0;
    }
}

pub struct MetricsHandler {
    current_frame: MetricsArray,
}

impl MetricsHandler {
    pub fn new() -> Self {
        Self {
            current_frame: MetricsArray::default(),
        }
    }

    pub fn record_sent_info(&mut self, sent_bytes: usize) {
        self.current_frame.sent_kbps.push(sent_bytes as f32/1000.0);
    }

    pub fn record_receive_info(&mut self, receive_bytes: usize) {
        self.current_frame.receive_kbps.push(receive_bytes as f32/1000.0);
    }

    pub fn record_packet_loss(&mut self, dropped_packets_count: usize) {
        self.current_frame.packet_loss += dropped_packets_count;
    }

    pub fn record_rtt(&mut self, rtt: f32) {
        self.current_frame.rtt.push(rtt);
    }

    // Should be called every second
    pub fn calculate_output(&mut self) -> Metrics {
        let sent_count = self.current_frame.sent_kbps.len();
        let receive_count = self.current_frame.receive_kbps.len();

        let result = Metrics {
            sent_packets: sent_count,
            received_packets: receive_count,
            sent_kbps: if sent_count == 0 {0.0} else {self.current_frame.sent_kbps.iter().fold(0.0, |acc, x| acc + x)/sent_count as f32},
            receive_kbps: if receive_count == 0 {0.0} else {self.current_frame.receive_kbps.iter().fold(0.0, |acc, x| acc + x)/receive_count as f32},
            packet_loss: if sent_count == 0 {0.0} else {self.current_frame.packet_loss as f32/sent_count as f32},
            rtt: 0.0,
        };

        self.current_frame.reset();
        
        result
    }
}