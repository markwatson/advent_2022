use euler::read_in_map;

type Instruction = (String, Option<i64>);

#[derive(Debug, Clone)]
struct VmState {
    reg_x: i64,
    cycle: i64,
    instruction: Instruction,
}

impl VmState {
    fn new() -> Self {
        VmState {
            reg_x: 1,
            cycle: 1,
            instruction: (String::from("init"), None),
        }
    }
}

#[derive(Debug, Clone)]
struct Vm<F>
where
    F: FnMut(&VmState),
{
    state: VmState,
    monitor: F,
}

impl<F> Vm<F>
where
    F: FnMut(&VmState),
{
    fn new(monitor: F) -> Self {
        Vm {
            state: VmState::new(),
            monitor,
        }
    }

    fn run(&mut self, instr: Instruction) {
        self.state.instruction = instr.clone();
        match instr.0.as_str() {
            "addx" => {
                (self.monitor)(&self.state);
                self.state.cycle += 1; // Incr
                (self.monitor)(&self.state);
                self.state.cycle += 1; // Incr
                self.state.reg_x += instr.1.expect("addx requires argument");
            }
            "noop" => {
                (self.monitor)(&self.state);
                self.state.cycle += 1; // Incr
            }
            _ => panic!("Unknown instruction: {}", instr.0),
        }
    }

    fn run_all(&mut self, instructions: Vec<Instruction>) {
        for instr in instructions {
            self.run(instr);
        }
    }
}

fn vm(instructions: Vec<Instruction>) -> i64 {
    let sprite_size = 3;
    let mut signal_step = 20;
    let mut total_signal = 0;

    // Stepper for screen output
    let mut screen_step = 0;

    let mut vm = Vm::new(|vm| {
        // Debug
        //println!("===========");
        //println!("Instruction: {:?}", vm.instruction);
        //println!("Cycle: {}", vm.cycle);
        //println!("Reg X: {}", vm.reg_x);

        // Step 1:
        if vm.cycle == signal_step {
            let signal = vm.cycle * vm.reg_x;
            // Debug
            //println!("Signal: c:{} * x:{} = {}", vm.cycle, vm.reg_x, signal);

            signal_step += 40;
            total_signal += signal;
        }

        // Step 2:
        let cycle_position = vm.cycle % 40;
        if cycle_position >= vm.reg_x && cycle_position <= (vm.reg_x + sprite_size - 1) {
            print!("#");
        } else {
            print!(".");
        }

        screen_step += 1;
        if screen_step % 40 == 0 {
            println!();
        }
    });
    vm.run_all(instructions);
    print!("\n\n");
    return total_signal;
}

fn main() {
    let fname = "./data/day_10";
    let re = r"(\w+)\s*(-?\d*)";
    let input = read_in_map(fname, re, |row| {
        (
            row[0].to_string(),
            row[1].parse::<i64>().map_or_else(|_| None, |f| Some(f)),
        )
    });
    // Debug
    //println!("Input: {:?}", input);

    let total_signal = vm(input);
    println!("Step 1: Total signal: {}", total_signal);
}
