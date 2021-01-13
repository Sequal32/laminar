#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Metrics {
    sent_packets: usize,
    received_packets: usize,
    sent_kbps: f32,
    receive_kbps: f32,
    packet_loss: f32,
    rtt: f32
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

    // Should be called every second
    pub fn calculate_output(&mut self) -> Metrics {
        let result = Metrics {
            sent_packets: self.current_frame.sent_kbps.len(),
            received_packets: self.current_frame.receive_kbps.len(),
            sent_kbps: self.current_frame.sent_kbps.iter().fold(0.0, |acc, x| acc + x)/self.current_frame.sent_kbps.len() as f32,
            receive_kbps: self.current_frame.receive_kbps.iter().fold(0.0, |acc, x| acc + x)/self.current_frame.sent_kbps.len() as f32,
            packet_loss: self.current_frame.packet_loss as f32/self.current_frame.sent_kbps.len() as f32,
            rtt: 0.0,
        };

        self.current_frame.reset();
        
        result
    }
}