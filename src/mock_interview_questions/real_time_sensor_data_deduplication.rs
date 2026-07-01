use std::cmp::Reverse;
use std::collections::BinaryHeap;
/**
 * I'm not sure what the terminology is, but we need a system that accepts
 * a time series of streamed events at a regular period and runs the analysis on
 * each chunk in a queue. So the stream is always coming in, and the analysis takes
 * place at a regular period. Within that analysis period, we perform deduplication
 * algorithms on the timeseriesed chunk of data. This processing would occur in parallel
 * to the data ingestion. The algo would have to execute quickly enough for the processing
 * to finish before the next data chunk arrives.
 *
 * So we need a few things:
 * - Some kind of server with a streaming endpoint. Maybe the server has an Mqtt or NATS
 * connection, in which case we would need to build a client for that.
 * - A dedubplication algorithm that runs at O(n) worst case. That would give us a more concrete
 * time-window to measure the data stream injestion vs algorithm race.
 * - A database with decent upload speed for deduplicated chunk loading. Or maybe we just dump
 * the data into an S3 bucket and store metadata of some kind to a sql table. Maybe the metadata
 * is the time series and record date etc.
 *
 * This backend service could live in a cloud environment like AWS. We can draw up a lo-fi
 * system diagram and then use terraform to model the system in a cloud provider infrastructure.
 *
 * The SLA would likely need to be 24/7. There are likely region considerations and tuning of the
 * deduplication chunk size depending on region in regards to upload speeds etc.
 *
 * Somthing like this could be tested via real-world tests. We could purposefully introduce
 * duplicated payload uploads and reason about how the service resolves them. Load testing would be important
 * as well. We could simulate a large number of streams like over 5000 coming in at once which all need
 * deduplicating.
 *
 */

/**
 * in order to solve deduplication, we need to know if we're seeing the same message
 * twice within a set elapsed time. To do this, we can create a hashmap which maps our
 * event_id to the event itself.
 *
 * the time window is the chunking mechanism i was imagining. we need to calculate a rolling
 * window as we process each event. when a window has elapsed, we would make a db call and flush
 * the timestamps that have been stored in memory.
 * */
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
struct Event {
    id: String,
    timestamp: u64,
    payload: String,
}

#[derive(Debug)]
struct StreamDeduplicator {
    window_delta: u64,
    eviction_queue: BinaryHeap<Reverse<(u64, String)>>, // min-heap
    events_map: HashMap<String, u64>,
}

impl StreamDeduplicator {
    fn new(window_ms: u64) -> Self {
        Self {
            window_delta: window_ms,
            eviction_queue: BinaryHeap::new(),
            events_map: HashMap::new(),
        }
    }

    fn perform_evictions(&mut self, latest_event_time: u64) {
        while let Some(entry) = self.eviction_queue.peek() {
            let oldest_entry_timestamp = entry.0.0;
            let oldest_entry_id = entry.0.clone().1;

            if oldest_entry_timestamp < latest_event_time.saturating_sub(self.window_delta) {
                self.eviction_queue.pop();
                if let Some(&stored_timestamp) = self.events_map.get(&oldest_entry_id) {
                    if stored_timestamp == oldest_entry_timestamp {
                        self.events_map.remove(&oldest_entry_id);
                    }
                }
            } else {
                break;
            }
        }
    }

    /// Returns Some(event) if it should pass through, None if it's a duplicate.
    fn process(&mut self, event: Event) -> Option<Event> {
        let incoming_event_id = event.id.clone();
        let incoming_event_timestamp = event.timestamp;

        self.perform_evictions(incoming_event_timestamp);

        if let Some(&old_event_entry_timestamp) = self.events_map.get(&incoming_event_id) {
            if (event.timestamp - old_event_entry_timestamp) < self.window_delta {
                return None;
            }

            self.events_map
                .insert(incoming_event_id.clone(), event.timestamp.clone());
            self.eviction_queue
                .push(Reverse((incoming_event_timestamp, incoming_event_id)));
            Some(event)
        } else {
            self.events_map
                .insert(incoming_event_id.clone(), event.timestamp.clone());
            self.eviction_queue
                .push(Reverse((incoming_event_timestamp, incoming_event_id)));
            Some(event)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_test_1() {
        let event_vec = vec![
            Event {
                id: "a".to_string(),
                timestamp: 1000,
                payload: "reading1".to_string(),
            },
            Event {
                id: "b".to_string(),
                timestamp: 2000,
                payload: "reading2".to_string(),
            },
            Event {
                id: "a".to_string(),
                timestamp: 2500,
                payload: "reading1".to_string(),
            },
            Event {
                id: "a".to_string(),
                timestamp: 7000,
                payload: "reading3".to_string(),
            },
            Event {
                id: "b".to_string(),
                timestamp: 6500,
                payload: "reading2".to_string(),
            },
            Event {
                id: "b".to_string(),
                timestamp: 7001,
                payload: "reading4".to_string(),
            },
        ];

        let processed_events_fixture = vec![
            Some(Event {
                id: "a".to_string(),
                timestamp: 1000,
                payload: "reading1".to_string(),
            }),
            Some(Event {
                id: "b".to_string(),
                timestamp: 2000,
                payload: "reading2".to_string(),
            }),
            None,
            Some(Event {
                id: "a".to_string(),
                timestamp: 7000,
                payload: "reading3".to_string(),
            }),
            None,
            Some(Event {
                id: "b".to_string(),
                timestamp: 7001,
                payload: "reading4".to_string(),
            }),
        ];

        // because the process function changes the struct itself
        let mut stream_duplicator = StreamDeduplicator::new(5000 as u64);

        let processed_events: Vec<Option<Event>> = event_vec
            .into_iter()
            .map(|e| stream_duplicator.process(e))
            .collect();

        assert_eq!(processed_events, processed_events_fixture)
    }
}
