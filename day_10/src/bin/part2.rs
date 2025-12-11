use z3::{Context, Optimize, SatResult, ast::Int};

#[derive(Debug)]
struct Controller {
    voltage: Vec<u32>,
    target_voltage: Vec<u32>,
    combos: Vec<Vec<u32>>,
}

impl Controller {
    fn new(line: &str) -> Self {
        let split_parts: Vec<&str> = line.split(" ").collect();
        let voltage_str = split_parts[split_parts.len() - 1];
        let combo_list = &split_parts[1..split_parts.len() - 1];

        let target_voltage: Vec<u32> = voltage_str[1..voltage_str.len() - 1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        Controller {
            voltage: vec![0; target_voltage.len()],
            target_voltage: target_voltage,
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

    // for each combo, ensure its pushed either 0 or more times
    let button_counts: Vec<Int> = (0..controller.combos.len())
        .map(|i| {
            let var = Int::new_const(format!("button_{}", i));
            opt.assert(&var.ge(&Int::from_u64(0)));
            var
        })
        .collect();

    // for each voltage position, calculate final voltage based on button presses
    for v_index in 0..controller.voltage.len() {
        let initial_voltage = controller.voltage[v_index];

        // start with initial voltage
        let mut voltage_final = Int::from_u64(initial_voltage as u64);

        // add voltage from each button press that affects this position
        for (combo_idx, combo) in controller.combos.iter().enumerate() {
            if combo.contains(&(v_index as u32)) {
                // each press of this button adds 1 to this voltage
                voltage_final = voltage_final + &button_counts[combo_idx];
            }
        }

        // add assertion that the voltage must match target
        opt.assert(&voltage_final.eq(&Int::from_u64(controller.target_voltage[v_index] as u64)));
    }

    // minimize total button presses
    let mut total = Int::from_i64(0);
    for count in &button_counts {
        total = total + count;
    }
    opt.minimize(&total);

    // check sat
    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let result = model.eval(&total, true).unwrap();
        return result.as_i64().unwrap() as u32;
    }

    0
}

fn part2(src_info: &str) -> u32 {
    let mut clicks = 0;

    let controllers: Vec<Controller> = src_info.split("\n").map(|x| Controller::new(x)).collect();
    for c in controllers.iter() {
        let ctx = Context::thread_local();
        let c_clicks = solve_controller(&ctx, &c);
        println!("clicks: {}", c_clicks);
        clicks += c_clicks;
    }

    return clicks;
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(result, 33);
    }
}
