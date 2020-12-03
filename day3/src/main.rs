mod file;
mod format;

fn main() {
    println!("2020 Day 2");
    println!("==========");

    part1();

    part2();
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn part1() {
    println!("\nPart 1:");

    let mut location = Point::new(0, 0);
    let slope = Point::new(3, 1);

    let tree_map: Vec<String> = file::lines().collect();
    let map_width = tree_map[0].len();
    let mut treecount = 0;
    while location.y < tree_map.len() as i64 {
        let found = tree_map[location.y as usize].as_bytes()[location.x as usize % map_width];
        let is_tree = if found == b'#' { "🎄" } else { "✖️" };
        println!("{:?} {}", location, is_tree);
        location.add(&slope);
        if is_tree == "🎄" {
            treecount += 1;
        }
    }

    println!("Solution Part 1: {}🎄", treecount);
}

fn part2() {
    println!("\nPart 2:");

    let tree_map: Vec<String> = file::lines().collect();
    let map_width = tree_map[0].len();

    let mut treecounts = 1;
    let mut slopes: Vec<Point> = vec![];
    slopes.push(Point::new(1, 1));
    slopes.push(Point::new(3, 1));
    slopes.push(Point::new(5, 1));
    slopes.push(Point::new(7, 1));
    slopes.push(Point::new(1, 2));

    for slope in slopes {
        let mut location = Point::new(0, 0);

        let mut treecount = 0;

        while location.y < tree_map.len() as i64 {
            let found = tree_map[location.y as usize].as_bytes()[location.x as usize % map_width];
            let is_tree = if found == b'#' { "🎄" } else { "✖️" };
            //        println!("{:?} {}", location,is_tree);
            location.add(&slope);
            if is_tree == "🎄" {
                treecount += 1;
            }
        }
        println!("Slope: {:?} - trees: {}", slope, treecount);

        treecounts *= treecount;
    }

    println!("Solution Part 2: {}🎄", treecounts);
}
