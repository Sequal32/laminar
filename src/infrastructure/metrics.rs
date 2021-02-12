use std::{
    cmp::min,
    ops::{Add, AddAssign},
};

const FACTOR: u32 = 2;
/// Metrics to be sent every second
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Metrics {
    /// Packets sent
    pub sent_packets: f32,
    /// Packets received
    pub received_packets: f32,
    /// The amount of data in kilobytes sent
    pub sent_kbps: f32,
    /// The amount of data in kilobytes received
    pub receive_kbps: f32,
    /// The percentage (0-1) packets lost
    pub packet_loss: f32,
    /// Round trip time
    pub rtt: f32,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            sent_packets: 0.0,
            received_packets: 0.0,
            sent_kbps: 0.0,
            receive_kbps: 0.0,
            packet_loss: 0.0,
            rtt: 0.0,
        }
    }
}

impl Add for Metrics {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            sent_packets: self.sent_packets + rhs.sent_packets,
            received_packets: self.received_packets + rhs.received_packets,
            sent_kbps: self.sent_kbps + rhs.sent_kbps,
            receive_kbps: self.receive_kbps + rhs.receive_kbps,
            packet_loss: self.packet_loss + rhs.packet_loss,
            rtt: self.rtt + rhs.rtt,
        }
    }
}

impl AddAssign for Metrics {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

pub struct MetricsHandler {
    current_frame: Metrics,
    current_averages: Metrics,
    counter: u32,
}

impl MetricsHandler {
    pub fn new() -> Self {
        Self {
            current_frame: Metrics::default(),
            current_averages: Metrics::default(),
            counter: 0,
        }
    }

    pub fn record_sent_info(&mut self, sent_bytes: usize) {
        self.current_frame.sent_kbps += sent_bytes as f32 / 1000.0;
        self.current_frame.sent_packets += 1.0;
    }

    pub fn record_receive_info(&mut self, receive_bytes: usize) {
        self.current_frame.receive_kbps += receive_bytes as f32 / 1000.0;
        self.current_frame.received_packets += 1.0;
    }

    pub fn record_packet_loss(&mut self, dropped_packets_count: usize) {
        self.current_frame.packet_loss += dropped_packets_count as f32;
    }

    pub fn record_rtt(&mut self, rtt: f32) {
        self.current_frame.rtt += rtt.abs(); // rtt can be negative for some reason
    }

    fn average(&self, new_value: f32, average: f32) -> f32 {
        return average + (new_value - average) / min(self.counter, FACTOR) as f32;
    }

    // Should be called every second
    pub fn calculate_output(&mut self) -> Metrics {
        self.counter += 1;
        self.current_averages = Metrics {
            sent_packets: self.average(
                self.current_frame.sent_packets,
                self.current_averages.sent_packets,
            ),
            received_packets: self.average(
                self.current_frame.received_packets,
                self.current_averages.received_packets,
            ),
            sent_kbps: self.average(
                self.current_frame.sent_kbps,
                self.current_averages.sent_kbps,
            ),
            receive_kbps: self.average(
                self.current_frame.receive_kbps,
                self.current_averages.receive_kbps,
            ),
            packet_loss: self.average(
                if self.current_frame.sent_packets > 0.0 {
                    self.current_frame.packet_loss / self.current_frame.sent_packets
                } else {
                    0.0
                },
                self.current_averages.packet_loss,
            ),
            rtt: self.average(self.current_frame.rtt, self.current_averages.rtt),
        };

        self.current_frame = Metrics::default();

        self.current_averages.clone()
    }
}
