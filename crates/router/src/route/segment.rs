// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub enum Segment {
    Satic(String),
    Dynamic(String),
    Tail(String),
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        // Define your custom comparison logic here
        match (self, other) {
            (Segment::Satic(a), Segment::Satic(b)) => a.cmp(b),
            (Segment::Satic(_), _) => Ordering::Less,
            (_, Segment::Satic(_)) => Ordering::Greater,
            (Segment::Dynamic(a), Segment::Dynamic(b)) => a.cmp(b),
            (Segment::Dynamic(_), _) => Ordering::Less,
            (_, Segment::Dynamic(_)) => Ordering::Greater,
            (Segment::Tail(a), Segment::Tail(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod segment_tests {
    use super::*;

    #[test]
    fn test_segment_sort() {
        let mut segments = vec![
            Segment::Tail("[...all]".to_string()),
            Segment::Satic("fixed".to_string()),
            Segment::Dynamic("[id]".to_string()),
            Segment::Satic("sub".to_string()),
        ];

        segments.sort();

        let expected_order = vec![
            Segment::Satic("fixed".to_string()),
            Segment::Satic("sub".to_string()),
            Segment::Dynamic("[id]".to_string()),
            Segment::Tail("[...all]".to_string()),
        ];

        assert_eq!(segments, expected_order);
    }
}
