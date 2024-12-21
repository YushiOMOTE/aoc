use aoc24_util::fetch_input;
use std::collections::{HashMap, HashSet};

struct Map {
    map: Vec<Option<Plot>>,
    width: isize,
    height: isize,
}

impl Map {
    fn parse(text: &str) -> Self {
        let map: Vec<_> = text
            .trim()
            .split('\n')
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(move |(x, ch)| Some(Plot::new(x as isize, y as isize, ch)))
            })
            .flatten()
            .collect();
        let width = map.last().unwrap().unwrap().x + 1;
        let height = map.last().unwrap().unwrap().y + 1;

        Self { map, width, height }
    }

    fn get(&self, id: Id) -> Option<Plot> {
        let Id(x, y) = id;

        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.map[(x + y * self.width) as usize]
        } else {
            None
        }
    }

    fn visit(&mut self, x: isize, y: isize) {
        let w = self.width;
        self.map[(x + y * w) as usize] = None;
    }
}

// Treat top-left-most plot location as region id
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Id(isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Plot {
    x: isize,
    y: isize,
    plant: char,
}

impl Plot {
    fn new(x: isize, y: isize, plant: char) -> Self {
        Self { x, y, plant }
    }

    fn id(&self) -> Id {
        Id(self.x, self.y)
    }
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    plot_ids: HashSet<Id>,
}

impl Region {
    fn new(root_plot: Plot) -> Self {
        Self {
            area: 1,
            perimeter: 0,
            plot_ids: [root_plot.id()].into_iter().collect(),
        }
    }
}

fn find_regions(map: &mut Map) -> HashMap<Id, Region> {
    let mut regions = HashMap::new();

    let (w, h) = (map.width, map.height);
    for (x, y) in (0..h).map(|y| (0..w).map(move |x| (x, y))).flatten() {
        if let Some(root_plot) = map.get(Id(x, y)) {
            let region = regions
                .entry(root_plot.id())
                .or_insert(Region::new(root_plot));

            map.visit(x, y);

            let mut current_plots = vec![root_plot];

            while !current_plots.is_empty() {
                for current_plot in std::mem::take(&mut current_plots) {
                    for (xdiff, ydiff) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                        let neighbor_plot_id = Id(current_plot.x + xdiff, current_plot.y + ydiff);

                        if let Some(neighbor_plot) = map.get(neighbor_plot_id) {
                            if neighbor_plot.plant == current_plot.plant {
                                region.area += 1;
                                region.plot_ids.insert(neighbor_plot.id());

                                map.visit(neighbor_plot.x, neighbor_plot.y);

                                current_plots.push(neighbor_plot);
                            } else {
                                region.perimeter += 1;
                            }
                        } else {
                            if !region.plot_ids.contains(&neighbor_plot_id) {
                                region.perimeter += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    regions
}

fn region_price(region: &Region) -> usize {
    region.area * region.perimeter
}

fn price(regions: &HashMap<Id, Region>) -> usize {
    regions.iter().map(|(_, r)| region_price(r)).sum()
}

fn main() {
    let text = fetch_input(12);

    let mut map = Map::parse(&text);
    let regions = find_regions(&mut map);

    println!("{}", price(&regions));
}
