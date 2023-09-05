// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Segment {
    /// A static segment in the URL path.
    /// Static segments are fixed and don't contain any parameter.
    /// Example: Segment::Static("fixed").
    Static(String),

    /// A dynamic segment in the URL path.
    /// Dynamic segments can contain a parameter that can change.
    /// Example: Segment::Dynamic("[id]").
    Dynamic(String),

    /// A trailing segment in the URL path.
    /// Trailing segments are used to match the remainder of the path after a certain point.
    /// Example: Segment::Tail("[...all]").
    Tail(String),
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Segment::Static(a), Segment::Static(b)) => a.cmp(b),
            (Segment::Static(_), _) => Ordering::Less,
            (_, Segment::Static(_)) => Ordering::Greater,
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

impl From<&str> for Segment {
    fn from(segment: &str) -> Self {
        if segment.starts_with("[...") {
            Segment::Tail(segment.to_owned())
        } else if segment.contains('[') {
            Segment::Dynamic(segment.to_owned())
        } else {
            Segment::Static(segment.to_owned())
        }
    }
}

#[cfg(test)]
mod segment_tests {
    use super::*;

    #[test]
    fn test_segment_sort() {
        let mut segments = vec![
            Segment::Tail("[...all]".to_string()),
            Segment::Static("fixed".to_string()),
            Segment::Dynamic("[id]".to_string()),
            Segment::Static("sub".to_string()),
        ];

        segments.sort();

        let expected_order = vec![
            Segment::Static("fixed".to_string()),
            Segment::Static("sub".to_string()),
            Segment::Dynamic("[id]".to_string()),
            Segment::Tail("[...all]".to_string()),
        ];

        assert_eq!(segments, expected_order);
    }

    #[test]
    fn test_segment_from_static() {
        let static_segment_str = "fixed";
        let static_segment = Segment::from(static_segment_str);
        assert_eq!(static_segment, Segment::Static("fixed".to_owned()));
    }

    #[test]
    fn test_segment_from_dynamic() {
        let dynamic_segment_str = "[id]";
        let dynamic_segment = Segment::from(dynamic_segment_str);
        assert_eq!(dynamic_segment, Segment::Dynamic("[id]".to_owned()));
    }

    #[test]
    fn test_segment_from_tail() {
        let tail_segment_str = "[...all]";
        let tail_segment = Segment::from(tail_segment_str);
        assert_eq!(tail_segment, Segment::Tail("[...all]".to_owned()));
    }
}
