// Day 6: Chronal Coordinates //
use lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct Marker {
    id: usize,
    x: usize,
    y: usize,
}

pub struct Point {
    is_marker: bool,
    is_edge: bool,
    closest_to: Option<usize>,
    total_distance: bool,
}

pub struct Grid {
    points: Vec<Point>,
}

impl Grid {
    pub fn find_area(self) -> usize {
        let mut dc_markers = vec![];
        let mut areas = HashMap::new();
        for point in self.points {
            match point.closest_to {
                None => {
                    continue;
                }
                Some(marker_id) => {
                    if point.is_edge {
                        dc_markers.push(point.closest_to.unwrap())
                    }
                    if !point.is_marker {
                        *areas.entry(marker_id).or_insert(1) += 1;
                    }
                }
            }
        }
        areas.iter().fold(0, |mut state, (&id, &area)| {
            if !dc_markers.contains(&id) {
                if state < area {
                    state = area
                }
            }
            state
        })
    }
}

pub struct PointBuilder {
    x: usize,
    y: usize,
}

impl PointBuilder {
    pub fn new(self, markers: &[Marker]) -> Point {
        Point {
            closest_to: self.closest_to(markers),
            is_edge: self.is_edge(grid_coords_finder(markers)),
            is_marker: self.is_marker(markers),
            total_distance: self.total_distance(markers),
        }
    }
    pub fn closest_to(&self, markers: &[Marker]) -> Option<usize> {
        let mut id_of_closest_marker: Option<usize> = None;
        let mut smallest_distance: usize = 500;
        for marker in markers {
            let manh_distance = ((self.x as i32 - marker.x as i32).abs()
                + (self.y as i32 - marker.y as i32).abs()) as usize;
            if smallest_distance > manh_distance {
                id_of_closest_marker = Some(marker.id);
                smallest_distance = ((self.x as i32 - marker.x as i32).abs()
                    + (self.y as i32 - marker.y as i32).abs())
                    as usize;
            } else if smallest_distance == manh_distance {
                id_of_closest_marker = None;
            }
        }
        id_of_closest_marker
    }

    pub fn is_edge(&self, coords: (usize, usize, usize, usize)) -> bool {
        if self.x == coords.0 && self.x == coords.1 {
            return true;
        } else if self.y == coords.2 && self.y == coords.3 {
            return true;
        }
        false
    }

    pub fn is_marker(&self, markers: &[Marker]) -> bool {
        let mut result = false;
        for marker in markers {
            if self.x == marker.x && self.y == marker.y {
                result = true
            }
        }
        result
    }

    pub fn total_distance(&self, markers: &[Marker]) -> bool {
        let mut running_total = 0;
        for marker in markers {
            let manh_distance = ((self.x as i32 - marker.x as i32).abs()
                + (self.y as i32 - marker.y as i32).abs()) as usize;
            if running_total + manh_distance < 10000 {
                running_total += manh_distance;
            } else {
                return false;
            }
        }
        return true;
    }
}

//Finds min/max x and min/max y coords for the grid
//Returns (min_x, max_x, min_y, max_y)
fn grid_coords_finder(input: &[Marker]) -> (usize, usize, usize, usize) {
    input
        .iter()
        .map(|marker| (marker.x, marker.y))
        .fold((132, 132, 308, 308), |mut state, item| {
            if item.0 <= state.0 {
                state.0 = item.0;
            } else if item.0 >= state.1 {
                state.1 = item.0;
            }
            if item.1 <= state.2 {
                state.2 = item.1;
            } else if item.1 >= state.3 {
                state.3 = item.1;
            }
            state
        })
}

pub fn instantiate_points(markers: &[Marker]) -> Vec<Point> {
    let coords = grid_coords_finder(markers);
    let mut points: Vec<Point> = vec![];
    for x in coords.0..=coords.1 {
        for y in coords.2..=coords.3 {
            let point = PointBuilder { x, y }.new(markers);
            points.push(point);
        }
    }
    points
}

#[aoc_generator(day6)]
pub fn input_marker(input: &str) -> Vec<Marker> {
    lazy_static::lazy_static! {
        static ref RE: Regex =
        Regex::new(r"(\d+), (\d+)").unwrap();
    };

    let collected: Vec<&str> = input.lines().collect();
    collected
        .iter()
        .map(|l| {
            let m_cap: Vec<&str> = RE
                .captures(l)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().as_str())
                .collect();
            let id: usize = (m_cap[1].to_owned() + &m_cap[2].to_owned())
                .parse()
                .unwrap();
            let x: usize = m_cap[1].parse().unwrap();
            let y: usize = m_cap[2].parse().unwrap();
            Marker { id, x, y }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Marker]) -> usize {
    let points = instantiate_points(input);
    Grid { points }.find_area()
}

#[aoc(day6, part2)]
pub fn part2(input: &[Marker]) -> usize {
    let points = instantiate_points(input);
    points.iter().filter(|x| x.total_distance).count()
}
