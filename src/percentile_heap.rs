use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::fmt;
use std::collections::HashSet;


const DEFAULT_HEAP_CAPACITY: usize = 2_usize.pow(5); // probably change to 2**16

pub struct PercentileHeap {
    percentile: f64,
    heap: BinaryHeap<OrderedFloat<f64>>,
    values_processed: usize,
    min_value: f64,
    max_value: f64
}

impl PercentileHeap {
    pub fn new(percentile: f64) -> Self {
        Self {
            percentile,
            heap: BinaryHeap::with_capacity(DEFAULT_HEAP_CAPACITY),
            values_processed: 0,
            min_value: f64::MAX,
            max_value: -f64::MAX
        }
    }

    pub fn insert(&mut self, value: f64) {
        self.values_processed += 1;
        let of_value: OrderedFloat<f64> = OrderedFloat(value);
        if self.should_expand() {
            self.heap.push(of_value);
        } else {
            if of_value < self.get_boundary() {
                self.heap.pop();
                self.heap.push(of_value);
            }
        }
        self.min_value = self.min_value.min(value);
        self.max_value = self.max_value.max(value);
    }

    pub fn get_boundary(&self) -> OrderedFloat<f64> {
        *self.heap.peek().unwrap()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn get_heap_str(&self) -> String {
        let elements: Vec<String> = self.heap.iter().map(|x| x.to_string()).collect();
        elements.join(", ")
    }

    pub fn is_at_threshold_for_expansion(&self) -> bool {
        self.get_heap_size_threshold() != ((self.values_processed + 2) as f64 * self.percentile).ceil() as usize
    }

    pub fn should_expand(&self) -> bool {
        self.heap.len() < self.get_heap_size_threshold()
    }

    fn get_heap_size_threshold(&self) -> usize {
        ((self.values_processed + 1) as f64 * self.percentile).ceil() as usize
    }

    pub fn smallest_values_processed_for_next_expansion(&self) -> usize {
        self.values_processed + self.get_delta_to_the_next_expansion()
    }

    pub fn get_delta_to_the_next_expansion(&self) -> usize {
        let expansion_step = (1f64 / self.percentile).ceil() as usize;
        let delta_to_the_prev_expansion = &self.values_processed  % expansion_step;
        expansion_step - delta_to_the_prev_expansion
    }

}

impl fmt::Display for PercentileHeap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PercentileHeap (values_processed: {}, min_value: {}, max_value: {}, percentile: {}, len: {})", self.values_processed, self.min_value, self.max_value, self.percentile, self.len())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::PercentileHeap;

    #[test]
    fn test_insert_and_boundary() {
        let range = (0..15);
        let percentile: f64 = 0.17;

        let mut pheap = PercentileHeap::new(percentile);
        for i in range.rev() {
            pheap.insert(i as f64);
            println!("values_processed {}, next_expansion {}, is_at_threshold_for_expansion {}, heap {}", pheap.values_processed, pheap.smallest_values_processed_for_next_expansion(), pheap.is_at_threshold_for_expansion(), pheap.get_heap_str());
        }

        println!("{}", pheap);
        // assert_eq!(pheap.len(), 4);

        //assert_eq!(pheap.get_boundary(), 19.0);

    }

    #[test]
    fn test_expansion() {
        test_expansion_helper(0..14, 0.25, HashSet::from([4, 8, 12, 16]));
        test_expansion_helper(0..32, 0.1, HashSet::from([10, 20, 30, 40]));
        test_expansion_helper(0..9, 0.5, HashSet::from([2, 4, 6, 8, 10]));
    }

    fn test_expansion_helper(range: std::ops::Range<usize>, percentile: f64, should_expand_at: HashSet<usize>) {
        let mut pheap = PercentileHeap::new(percentile);
        for value in range.rev() {
            pheap.insert(value as f64);

            // Your existing test logic
            if should_expand_at.contains(&(pheap.values_processed + 1)) {
                assert!(pheap.is_at_threshold_for_expansion());
            }
            assert!(should_expand_at.contains(&pheap.smallest_values_processed_for_next_expansion()));

            if pheap.is_at_threshold_for_expansion() {
                assert_eq!(&(pheap.values_processed + 1), &pheap.smallest_values_processed_for_next_expansion());
            }
        }
    }
}
