use std::collections::HashSet;

use crate::solution::Solution;

pub struct Playground;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

type Tree = HashSet<Point>;

#[derive(Debug, Clone, Copy)]
struct Edge {
    a: Point,
    b: Point,
    distance: f64,
}

fn generate_edges(points: &[Point]) -> Vec<Edge> {
    let mut edges = Vec::new();
    for (i, a) in points.iter().enumerate() {
        for b in points[i + 1..].iter() {
            edges.push(Edge {
                a: *a,
                b: *b,
                distance: a.distance(&b),
            });
        }
    }
    edges
}

fn find_tree<'a>(forest: &'a [Tree], point: &'a Point) -> usize {
    forest
        .iter()
        .enumerate()
        .find(|(_, t)| t.contains(point))
        .unwrap()
        .0
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point::new(parts[0], parts[1], parts[2])
        })
        .collect()
}

fn kruskals_n_edges(points: &[Point], n: usize) -> (Vec<Tree>, Vec<Edge>) {
    let mut edges = generate_edges(points);
    edges.sort_by(|x, y| x.distance.total_cmp(&y.distance));

    // Build the set of edges sorted by length asc
    let mut forest: Vec<Tree> = points
        .iter()
        .cloned()
        .map(|point| std::iter::once(point).collect())
        .collect();

    // Kruskals
    // for edges shortest -> longest
    let mut min_edges: Vec<Edge> = Vec::new();
    for edge in edges.iter().take(n) {
        // if edge links two trees, put them together
        let i = find_tree(&forest, &edge.a);
        let j = find_tree(&forest, &edge.b);
        if i == j {
            continue;
        }

        // Remove the larger of the two indices first so the smaller stays valid
        min_edges.push(*edge);
        let t1 = forest.remove(i.max(j));
        let t2 = forest.remove(i.min(j));
        forest.push(t1.union(&t2).cloned().collect());

        if forest.len() == 1 {
            break;
        }
    }

    forest.sort_by(|x, y| y.len().cmp(&x.len()));

    (forest, min_edges)
}

impl Solution for Playground {
    fn part1(puzzle_input: String) -> String {
        let points = parse_input(&puzzle_input);
        let (forest, _) = kruskals_n_edges(&points, 1000);
        forest
            .iter()
            .map(|t| t.len())
            .take(3)
            .product::<usize>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let points = parse_input(&puzzle_input);
        let (_, min_edges) = kruskals_n_edges(&points, points.len() * points.len());
        let last_edge = min_edges.last().unwrap();
        (last_edge.a.x * last_edge.b.x).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_1() {
        let points = parse_input(TEST_INPUT);
        let (forest, _) = kruskals_n_edges(&points, 10);
        let answer: usize = forest.iter().map(|t| t.len()).take(3).product();
        assert_eq!(answer, 40)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Playground::part2(TEST_INPUT.to_owned()), "25272");
    }
}
