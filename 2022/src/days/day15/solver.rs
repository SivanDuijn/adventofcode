// https://adventofcode.com/2022/day/15

struct Ranges {
    ranges: Vec<Range>,
}
impl Ranges {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn dist(&self) -> i32 {
        self.ranges.iter().map(|r| r.len()).sum()
    }

    pub fn one_space_between(&self) -> Option<i32> {
        if self.ranges.len() != 2 { return None; }

        if self.ranges[1].start - self.ranges[0].end == 2 {
            return Some(self.ranges[0].end + 1);
        }

        return None;
    }

    pub fn insert(&mut self, start: i32, end: i32) {
        // Check if ranges are overlapping
        let mut i = 0;

        if self.ranges.is_empty() {
            self.ranges.push(Range::new(start, end));
            return;
        }

        if end + 1 < self.ranges.first().unwrap().start {
            self.ranges.insert(0, Range::new(start, end));
            return;
        }
        else if start - 1 > self.ranges.last().unwrap().end {
            self.ranges.push(Range::new(start, end));
            return;
        }
        

        while i < self.ranges.len() {
            if i + 1 < self.ranges.len() &&
              start - 1 > self.ranges[i].end && end + 1 < self.ranges[i + 1].start {
                 self.ranges.insert(i + 1, Range::new(start, end));
                 return;
            }

            let range = &mut self.ranges[i];

            // Inserted range already fully contained in another range
            if start >= range.start && end <= range.end { return; }
            
            if start < range.start && end + 1 >= range.start {
                range.start = start;
                if end <= range.end { return }
                if start - 1 <= range.end && end > range.end {
                    range.end = end;
                    break;
                }
            }
            if start - 1 <= range.end && end > range.end {
                range.end = end;
                break;
            }

            i += 1;
        }

        let j = i;
        i += 1;
        while i < self.ranges.len() {
            if self.ranges[j].end >= self.ranges[i].end {
                self.ranges.remove(i);
                continue;
            }

            if self.ranges[j].end + 1 >= self.ranges[i].start {
                self.ranges[j].end = self.ranges[i].end;
                self.ranges.remove(i);
                return;
            }

            i += 1;
        }

    }
}

struct Range {
    start: i32,
    end: i32,
}
impl Range {
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
    pub fn len(&self) -> i32 {
        return self.end - self.start;
    }
}

struct Sensor {
    x: i32,
    y: i32,
    dist_to_closest_beacon: i32
}

fn parse(input: &str) -> Vec<Sensor> {
    input.split("\n").map(|line| {
        let coords: Vec<Vec<i32>> = line
        .split(":")
        .map(|s| 
            s.split(",").map(|s| 
                s.split("=").last().unwrap().parse().unwrap()
            ).collect()
        ).collect();
        let x: i32 = coords[0][0];
        let y: i32 = coords[0][1];
        let dist: i32 = (x - coords[1][0]).abs() + (y - coords[1][1]).abs();
        Sensor { x, y, dist_to_closest_beacon: dist }
    }).collect()
}

fn create_ranges(sensors: &Vec<Sensor>, y: i32) -> Ranges {
    let mut ranges: Ranges = Ranges::new();
    sensors.iter()
        .filter(|s| (s.y - y).abs() <= s.dist_to_closest_beacon)
        .for_each(|s| {
            let l = s.dist_to_closest_beacon - (s.y - y).abs();
            let start = s.x - l;
            let end = s.x + l;
            ranges.insert(start, end);
        });

    return ranges;
} 

fn solve1(sensors: &Vec<Sensor>) -> String {
    let y = 2000000; //10; // for example solution

    create_ranges(sensors, y).dist().to_string()
}

fn solve2(sensors: &Vec<Sensor>) -> String {
    let y_center: i32 = sensors.iter().map(|s| s.y).sum::<i32>() / (sensors.len() as i32);

    let mut y_up = y_center;
    let mut y_down = y_center + 1;

    loop {
        if let Some(x) = create_ranges(sensors, y_up).one_space_between() {
            return (x as i128 * 4000000 + y_up as i128).to_string();
        }
        if let Some(x) = create_ranges(sensors, y_down).one_space_between() {
            return (x as i128 * 4000000 + y_down as i128).to_string();
        }

        y_up -= 1;
        y_down += 1;
    }
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}