// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Represents the type of a route.
///
/// - `Static`: Represents a static route with a fixed number of segments.
/// - `Dynamic`: Represents a dynamic route with a fixed number of segments.
/// - `Tail`: Represents a tail route with a variable number of segments. It may also contain dynamic segments.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum RouteType {
    Static { number_of_segments: usize },
    Dynamic { number_of_segments: usize },
    Tail { number_of_segments: usize },
}
impl From<&String> for RouteType {
    fn from(route_path: &String) -> Self {
        let number_of_segments = route_path.chars().filter(|&c| c == '/').count();
        if route_path.contains("/[...") {
            RouteType::Tail { number_of_segments }
        } else if route_path.contains("/[") {
            RouteType::Dynamic { number_of_segments }
        } else {
            RouteType::Static { number_of_segments }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_type_from_string_static() {
        let route_path = String::from("/fixed");
        let route_type = RouteType::from(&route_path);

        assert_eq!(
            route_type,
            RouteType::Static {
                number_of_segments: 1
            }
        );
    }

    #[test]
    fn route_type_from_string_dynamic() {
        let route_path = String::from("/[id]");
        let route_type = RouteType::from(&route_path);

        assert_eq!(
            route_type,
            RouteType::Dynamic {
                number_of_segments: 1
            }
        );
    }

    #[test]
    fn route_type_from_string_tail() {
        let route_path = String::from("/sub/[...all]");
        let route_type: RouteType = RouteType::from(&route_path);

        assert_eq!(
            route_type,
            RouteType::Tail {
                number_of_segments: 2
            }
        );
    }

    #[test]
    fn route_type_from_string_unknown() {
        let route_path = String::from("/unknown");
        let route_type: RouteType = RouteType::from(&route_path);

        // Default to static if the format is not recognized
        assert_eq!(
            route_type,
            RouteType::Static {
                number_of_segments: 1
            }
        );
    }
}
