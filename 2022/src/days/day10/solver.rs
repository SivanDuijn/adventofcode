// https://adventofcode.com/2022/day/10

fn parse(input: &str) -> Vec<Vec<&str>> {
    input.split("\n")
    .map(|l| l.split(" ").collect())
    .collect()
}

struct CRT {
    cycle_counter: i32,
    register_x: i32,
    next_row_cycle: i32,
    total_signal_strength: i32,
    screen: String,
    draw_to_screen: bool,
}

impl CRT {
    pub fn new(next_row_cycle: i32, draw_to_screen: bool) -> Self {
        Self {
            cycle_counter: 0,
            register_x: 1,
            next_row_cycle,
            total_signal_strength: 0,
            screen: "\n".to_string(),
            draw_to_screen
        }
    }

    pub fn addx(&mut self, x: i32) {
        self.tick();
        self.tick();
        self.register_x += x;
    }
    
    pub fn noop(&mut self) {
        self.tick();
    }

    fn tick(&mut self) {
        if self.draw_to_screen {
            self.draw_pixel();
        }

        self.cycle_counter += 1;
        if self.cycle_counter == self.next_row_cycle {
            self.next_row_cycle += 40;

            self.total_signal_strength += self.register_x * self.cycle_counter;
            self.screen += "\n";
        }
    }

    fn draw_pixel(&mut self) {
        if ((self.cycle_counter % 40 - self.register_x) as i32).abs() <= 1 {
            self.screen += "#";
        }
        else {
            self.screen += ".";
        }
    }
}

fn execute_crt_program(crt: &mut CRT, program: &Vec<Vec<&str>>) {
    for cmd in program {
        match cmd[0] {
            "addx" => crt.addx(cmd[1].parse::<i32>().unwrap()),
            _      => crt.noop()
        }
    }
}

fn solve1(program: &Vec<Vec<&str>>) -> String {
    let crt = &mut CRT::new(20, false);
    execute_crt_program(crt, program);
    
    return crt.total_signal_strength.to_string();
}

fn solve2(program: &Vec<Vec<&str>>) -> String {
    let crt = &mut CRT::new(40, true);
    execute_crt_program(crt, program);

    return crt.screen.to_owned();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed_input = parse(input);
    return (solve1(&parsed_input), solve2(&parsed_input));
}