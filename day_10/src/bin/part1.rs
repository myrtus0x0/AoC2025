use z3::{
    Context, Optimize, SatResult,
    ast::{Bool, Int},
};

#[derive(Debug)]
struct Controller {
    lights: Vec<bool>,
    target_lights: Vec<bool>,
    combos: Vec<Vec<u32>>,
}

impl Controller {
    fn new(line: &str) -> Self {
        let split_parts: Vec<&str> = line.split(" ").collect();
        let start_state = split_parts[0];
        let combo_list = &split_parts[1..&split_parts.len() - 1];

        let mut target_lights = vec![];
        for ch in start_state.chars().skip(1).take(start_state.len() - 2) {
            if ch == '.' {
                target_lights.push(false);
            } else {
                target_lights.push(true);
            }
        }

        Controller {
            target_lights: target_lights,
            lights: vec![false; start_state.len() - 2],
            combos: combo_list
                .iter()
                .map(|x| {
                    x[1..x.len() - 1]
                        .split(",")
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

fn solve_controller(_ctx: &Context, controller: &Controller) -> u32 {
    let opt = Optimize::new();

    // each button var represents one combo
    let button_vars: Vec<Bool> = (0..controller.combos.len())
        .map(|i| Bool::new_const(format!("button_{}", i)))
        .collect();

    // create initial state for each light, and create the XOR constraint
    for light_idx in 0..controller.lights.len() {
        let initial_state = controller.lights[light_idx];
        // create initial state for our light as z3 boolean
        let mut light_final = Bool::from_bool(initial_state);

        // xor with each button that affects this light
        for (combo_idx, combo) in controller.combos.iter().enumerate() {
            for i in combo {
                if *i == light_idx as u32 {
                    light_final = light_final.xor(&button_vars[combo_idx]);
                }
            }
        }

        // add assertion that the light must match target
        if controller.target_lights[light_idx] {
            opt.assert(&light_final);
        } else {
            opt.assert(&!light_final);
        }
    }

    // convert bool to int
    let mut button_ints: Vec<Int> = vec![];
    for b in &button_vars {
        button_ints.push(b.ite(&Int::from_i64(1), &Int::from_i64(0)));
    }

    // get solution and optimize solution
    let total = button_ints.iter().fold(Int::from_i64(0), |acc, i| acc + i);
    opt.minimize(&total);

    // check sat
    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let result = model.eval(&total, true).unwrap();
        return result.as_i64().unwrap() as u32;
    }

    0
}

fn part1(src_info: &str) -> u32 {
    let mut clicks = 0;
    let ctx = Context::thread_local();

    let controllers: Vec<Controller> = src_info.split("\n").map(|x| Controller::new(x)).collect();
    for (i, c) in controllers.iter().enumerate() {
        let c_clicks = solve_controller(&ctx, &c);
        println!("controller {} clicks: {}", i, c_clicks);
        clicks += c_clicks;
    }

    return clicks;
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part1(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part1(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(result, 7);
    }
}
