use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Packet {
    source: i32,
    destination: i32,
    timestamp: i32,
}

struct Router {
    queue: VecDeque<Packet>,
    packets: HashSet<Packet>,
    memory_limit: usize,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Router {
            queue: VecDeque::new(),
            packets: HashSet::new(),
            memory_limit: memory_limit as usize,
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = Packet {
            source,
            destination,
            timestamp,
        };
        if self.packets.contains(&packet) {
            false
        } else {
            if self.queue.len() == self.memory_limit {
                let oldest = self.queue.pop_front().unwrap();
                self.packets.remove(&oldest);
            }
            self.packets.insert(packet);
            self.queue.push_back(packet);
            true
        }
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        let oldest = self.queue.pop_front();
        match oldest {
            Some(packet) => {
                self.packets.remove(&packet);
                vec![packet.source, packet.destination, packet.timestamp]
            }
            None => vec![],
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if self.queue.is_empty() {
            return 0;
        }

        let start_idx = self.queue.partition_point(|x| x.timestamp < start_time);
        let end_idx = self.queue.partition_point(|x| x.timestamp <= end_time);

        if start_idx >= end_idx {
            return 0;
        }
        self.queue
            .range(start_idx..end_idx)
            .filter(|packet| packet.destination == destination)
            .count() as i32
    }
}

fn main() {
    use utils::prelude::*;
    init_logger();
    let mut router = Router::new(3);
    router.add_packet(1, 4, 90);
    router.add_packet(2, 5, 90);
    router.add_packet(1, 4, 90);
    router.add_packet(3, 5, 95);
    router.add_packet(4, 5, 105);
    tracing::info!("{:?}", router.forward_packet());
    router.add_packet(5, 2, 110);
    tracing::info!("{:?}", router.get_count(5, 100, 110));
}
