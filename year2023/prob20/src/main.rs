use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

// A signal is a tuple of a source, high/low and destination
type Signal = (String, bool, String);
type Modules = HashMap<String, Box<dyn IO>>;

trait IO: std::fmt::Debug {
    fn input(&mut self, high_pulse: bool, source: Option<&str>) -> Option<Vec<Signal>>;
    fn add_input(&mut self, input: &str);
    fn add_connection(&mut self, connection: &str);
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: bool,
    connections: Vec<String>,
}

impl FlipFlop {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: false,
            connections: vec![],
        }
    }
}

impl IO for FlipFlop {
    /// A FlipFlop will flip it's state on low pulse
    fn input(&mut self, high_pulse: bool, _source: Option<&str>) -> Option<Vec<Signal>> {
        if !high_pulse {
            self.state = !self.state;
            Some(
                self.connections
                    .iter()
                    .map(|connection| (self.name.clone(), self.state, connection.to_string()))
                    .collect(),
            )
        } else {
            None
        }
    }

    fn add_connection(&mut self, connection: &str) {
        self.connections.push(connection.to_string());
    }

    fn add_input(&mut self, _input: &str) {}

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Default, Debug)]
struct Conjunction {
    name: String,
    states: HashMap<String, bool>,
    connections: Vec<String>,
}

impl Conjunction {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            states: HashMap::new(),
            connections: vec![],
        }
    }
}

impl IO for Conjunction {
    /// A Conjunction will remember the most recent pulse for each connection. When a pulse is
    /// received, the conjunction module first updates its memory for that input. Then, if it
    /// remembers high pulse for all inputs, it sends a low pulse; otherwise, it sends a high pulse
    fn input(&mut self, high_pulse: bool, source: Option<&str>) -> Option<Vec<Signal>> {
        let source = source.unwrap();
        if !self.states.contains_key(source) {
            panic!("[{}] Unknown input: {}", self.name, source);
        }
        self.states.insert(source.to_string(), high_pulse);

        let output = !self.states.values().all(|&state| state);
        Some(
            self.connections
                .iter()
                .map(|connection| (self.name.clone(), output, connection.to_string()))
                .collect(),
        )
    }

    fn add_input(&mut self, input: &str) {
        self.states.insert(input.to_string(), false);
    }

    fn add_connection(&mut self, connection: &str) {
        self.connections.push(connection.to_string());
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
struct Broadcaster {
    name: String,
    connections: Vec<String>,
}

impl Broadcaster {
    fn new() -> Self {
        Self {
            name: "broadcaster".to_string(),
            connections: vec![],
        }
    }
}

impl IO for Broadcaster {
    fn input(&mut self, high_pulse: bool, _source: Option<&str>) -> Option<Vec<Signal>> {
        Some(
            self.connections
                .iter()
                .map(|connection| (self.name.clone(), high_pulse, connection.to_string()))
                .collect(),
        )
    }

    fn add_connection(&mut self, connection: &str) {
        self.connections.push(connection.to_string());
    }

    fn add_input(&mut self, _input: &str) {}

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    // Lines will start with:
    //  * 'broadcaster': A single line that is the main broadcaster
    //  * %: A FlipFlop
    //  * &: A Conjunction
    //
    // Lines will look like this:
    //  broadcaster -> a, b, c
    //  %a -> b
    //  &inv -> a
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.sort_by_key(|line| match line.chars().next().unwrap() {
        '&' => 0,
        '%' => 1,
        _ => 2,
    });

    // Do a first pass to get all conjunctions and store them in a HashMap. The value will be a
    // vector of inputs to the conjunction, which we will populate as we create the modules
    // After all modules have been created, we can update the conjunctions to set the inputs
    let mut conjunctions: HashMap<String, Vec<String>> = input
        .lines()
        .filter_map(|line| {
            if line.starts_with('&') {
                let conjunction_name = line.split(" -> ").next().unwrap();
                Some((conjunction_name[1..].to_string(), vec![]))
            } else {
                None
            }
        })
        .collect();

    let mut modules: Modules = HashMap::new();

    for line in lines {
        let mut parts = line.split(" -> ");
        let source = parts.next().unwrap();
        let destinations = parts.next().unwrap().split(", ");

        let mut module: Box<dyn IO> = match source.chars().next().unwrap() {
            'b' => Box::new(Broadcaster::new()),
            '%' => Box::new(FlipFlop::new(&source[1..])),
            '&' => Box::new(Conjunction::new(&source[1..])),
            other => panic!("Unknown module type: {}", other),
        };

        for destination in destinations {
            module.add_connection(destination);
            if conjunctions.contains_key(destination) {
                conjunctions
                    .get_mut(destination)
                    .unwrap()
                    .push(module.get_name().to_string())
            }
        }
        modules.insert(module.get_name().to_string(), module);
    }

    // Update conjunction inputs
    for (name, inputs) in conjunctions {
        let conjunction = modules.get_mut(&name).unwrap();
        for input in inputs {
            conjunction.add_input(&input);
        }
    }

    let mut signals: VecDeque<Signal> = VecDeque::new();
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    for _ in 1..=1000 {
        signals.push_back(("button".to_string(), false, "broadcaster".to_string()));
        while let Some((source, high_pulse, destination)) = signals.pop_front() {
            match high_pulse {
                true => high_pulse_count += 1,
                false => low_pulse_count += 1,
            }
            match modules.get_mut(&destination) {
                Some(module) => {
                    if let Some(new_signals) = module.input(high_pulse, Some(&source)) {
                        for signal in new_signals {
                            signals.push_back(signal);
                        }
                    }
                }
                None => {
                    // Output module
                }
            }
        }
    }

    Ok(low_pulse_count * high_pulse_count)
}

fn part2(_input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const TEST_INPUT2: &str = include_str!("../test2.txt");

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 32000000);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1(TEST_INPUT2).unwrap(), 11687500);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_modules() {
        let mut modules: Modules = HashMap::new();

        let mut broadcaster = Broadcaster::new();

        let mut flipflop_a = FlipFlop::new("a");
        let mut flipflop_b = FlipFlop::new("b");
        let mut flipflop_c = FlipFlop::new("c");

        broadcaster.add_connection(&flipflop_a.name);
        broadcaster.add_connection(&flipflop_b.name);
        broadcaster.add_connection(&flipflop_c.name);

        flipflop_a.add_connection(&flipflop_b.name);
        flipflop_b.add_connection(&flipflop_c.name);

        let mut inv = Conjunction::new("inv");

        flipflop_c.add_connection(&inv.name);

        inv.add_input(&flipflop_c.name);
        inv.add_connection(&flipflop_a.name);

        modules.insert(broadcaster.name.clone(), Box::new(broadcaster));
        modules.insert(flipflop_a.name.clone(), Box::new(flipflop_a));
        modules.insert(flipflop_b.name.clone(), Box::new(flipflop_b));
        modules.insert(flipflop_c.name.clone(), Box::new(flipflop_c));
        modules.insert(inv.name.clone(), Box::new(inv));

        let mut signals: VecDeque<Signal> =
            vec![("button".to_string(), false, "broadcaster".to_string())].into();
        let mut signal_count = 0;
        while let Some((source, high_pulse, destination)) = signals.pop_front() {
            signal_count += 1;
            if let Some(new_signals) = modules
                .get_mut(&destination)
                .unwrap()
                .input(high_pulse, Some(&source))
            {
                for signal in new_signals {
                    signals.push_back(signal);
                }
            }
        }

        assert_eq!(signal_count, 12);
    }
}
