use aoc_runner_derive::aoc;

struct Point{
    x: i32,
    y: i32
}
struct Instruction{
    light: (Point, Point),
    on: bool
}


impl Instruction {
    fn from_command(command: &str) -> Instruction {
        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts.as_slice() {
            ["toggle", start, "through", end] => {
                let start_coords: Vec<i32> = start.split(',').map(|s| s.parse().unwrap()).collect();
                let end_coords: Vec<i32> = end.split(',').map(|s| s.parse().unwrap()).collect();

                Instruction {
                    light: (
                        Point {
                            x: start_coords[0],
                            y: start_coords[1],
                        },
                        Point {
                            x: end_coords[0],
                            y: end_coords[1],
                        },
                    ),
                    on: false, // Explicitly "off"
                }
            }
            // Match "turn on"
            ["turn", "on", start, "through", end] => {
                let start_coords: Vec<i32> = start.split(',').map(|s| s.parse().unwrap()).collect();
                let end_coords: Vec<i32> = end.split(',').map(|s| s.parse().unwrap()).collect();

                Instruction {
                    light: (
                        Point {
                            x: start_coords[0],
                            y: start_coords[1],
                        },
                        Point {
                            x: end_coords[0],
                            y: end_coords[1],
                        },
                    ),
                    on: true, // Explicitly "on"
                }
            }
            // Match "turn off"
            ["turn", "off", start, "through", end] => {
                let start_coords: Vec<i32> = start.split(',').map(|s| s.parse().unwrap()).collect();
                let end_coords: Vec<i32> = end.split(',').map(|s| s.parse().unwrap()).collect();

                Instruction {
                    light: (
                        Point {
                            x: start_coords[0],
                            y: start_coords[1],
                        },
                        Point {
                            x: end_coords[0],
                            y: end_coords[1],
                        },
                    ),
                    on: false, // Explicitly "off"
                }
            }
            _ => panic!("Invalid command format"),
        }
    }
}



#[aoc_generator(day6)]
fn preprocess(input: &str) -> Vec<Instruction> {
    input.lines()
}
