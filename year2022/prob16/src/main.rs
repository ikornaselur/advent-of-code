use advent::prelude::*;
use parse::parse_input;

mod parse;

#[derive(Debug, PartialEq)]
struct Valve {
    id: usize,
    name: (char, char),
    flow_rate: usize,
    tunnels: Vec<(char, char)>,
    tunnel_ids: Option<Vec<usize>>,
}

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn get_valve_open_state(valve_id: usize, bitmask: u64) -> bool {
    (bitmask & (1 << valve_id)) != 0
}

fn set_valve_open_state(valve_id: usize, bitmask: u64, open: bool) -> u64 {
    if open {
        bitmask | (1 << valve_id)
    } else {
        bitmask & !(1 << valve_id)
    }
}

fn get_open_valves_for_bitmask(valve_map: &HashMap<usize, Valve>, bitmask: u64) -> Vec<&Valve> {
    valve_map
        .values()
        .filter(|v| get_valve_open_state(v.id, bitmask))
        .collect()
}

fn get_current_open_valve_flow_rate(valve_map: &HashMap<usize, Valve>, bitmask: u64) -> usize {
    get_open_valves_for_bitmask(valve_map, bitmask)
        .iter()
        .map(|v| v.flow_rate)
        .sum()
}

#[allow(clippy::too_many_arguments)]
/// Get the max pressure possible from the given state
///
/// When there are no steps left, there is no more pressure to be gotten
/// When there are steps left, but all valves are open, we can skip to the end (calculating the
/// total pressure at that point)
/// When there are any valves that can be opened, and there are steps left, we need to explore
/// navigating to those valves to open them
///
/// This can be broken down into:
///
/// 1. No steps left -> return 0
/// 2. Steps left, but all valves are open -> return steps_left times the total pressure released
///    per tick
/// 3. Recursively call the function with the state that will be at the point of hitting those
///    nodes
/// 4. Dynamic Programming stuff that involves some caching stuff
///
/// Random thoughts:
///
/// * Could I just calculate what the bitmask is for 'all valves with flowrate are open' and exit
///   early based on that?
fn get_max_pressure(
    dp: &mut HashMap<(usize, u64, usize), usize>,
    valve_map: &HashMap<usize, Valve>,
    valves_with_flowrate: &Vec<usize>,
    steps_left: usize,
    bitmask: u64,
    all_valves_open_bitmask: u64,
    max_flow_rate: usize,
    current_valve_id: usize,
) -> usize {
    // Step 1, no steps left, no more pressure!
    if steps_left == 0 {
        return 0;
    }

    // Step 2, all valves are open
    if bitmask == all_valves_open_bitmask {
        return steps_left * max_flow_rate;
    }

    // Step 4, dynamic programming stuff - we'll revisit this
    if let Some(&pressure) = dp.get(&(steps_left, bitmask, current_valve_id)) {
        return pressure;
    }

    // Step 3, recursively call the function for going to all unopened valves
    // We'll start with calculating the distance to all nodes from current.. this can be optimised,
    // but it's just a POC right now
    let mut distances = HashMap::new();
    let mut queue = VecDeque::from(vec![(current_valve_id, 0)]);
    while let Some((valve_id, dist)) = queue.pop_front() {
        if distances.contains_key(&valve_id) {
            continue;
        }
        distances.insert(valve_id, dist);
        let valve = valve_map.get(&valve_id).unwrap();
        for tunnel_id in valve.tunnel_ids.as_ref().unwrap() {
            queue.push_back((*tunnel_id, dist + 1));
        }
    }

    let mut valve_pressures = Vec::new();
    for valve in valves_with_flowrate {
        // If it's already opened, we just skip it
        if get_valve_open_state(*valve, bitmask) {
            continue;
        }
        // This can be optimised.. but we'll just visit all nodes until we've hit all closed valves
        let dist = distances.get(valve).unwrap();
        if dist + 1 > steps_left {
            continue;
        }
        // We calculate how much pressure will be released during the move (dist) plus 1 for the
        // time it'll take to open the valve after arriving there
        let pressure_during_move =
            get_current_open_valve_flow_rate(valve_map, bitmask) * (dist + 1);
        let new_bitmask = set_valve_open_state(*valve, bitmask, true);
        // And now let's calculate the pressure from that state
        let pressure = get_max_pressure(
            dp,
            valve_map,
            valves_with_flowrate,
            steps_left - dist - 1,
            new_bitmask,
            all_valves_open_bitmask,
            max_flow_rate,
            *valve,
        );

        // And we store it, so we can figure out the best case from all of these
        valve_pressures.push(pressure_during_move + pressure);
    }
    if valve_pressures.is_empty() {
        // We can only just skip to the end
        return steps_left * get_current_open_valve_flow_rate(valve_map, bitmask);
    }

    // Now let's get the max pressure of these so we know the best case for this current state
    let max_pressure = valve_pressures.into_iter().max().unwrap();

    // Step 4, cache the result, so if we happen to find this state again
    dp.insert((steps_left, bitmask, current_valve_id), max_pressure);

    max_pressure
}

fn part1(input: &str) -> Result<usize> {
    let mut valves = parse_input(input)?;
    let valve_name_to_id_map = valves
        .iter()
        .map(|v| (v.name, v.id))
        .collect::<HashMap<_, _>>();

    for valve in valves.iter_mut() {
        valve.tunnel_ids = Some(
            valve
                .tunnels
                .iter()
                .map(|t| *valve_name_to_id_map.get(t).unwrap())
                .collect(),
        );
    }

    let valve_map: HashMap<usize, Valve> = valves
        .into_iter()
        .map(|v| (v.id, v))
        .collect::<HashMap<_, _>>();

    let steps_left = 30;
    let valve_a_id = valve_map
        .iter()
        .find(|(_, v)| v.name == ('A', 'A'))
        .map(|(id, _)| *id)
        .unwrap();
    let valves_with_flowrate = valve_map
        .values()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.id)
        .collect::<Vec<_>>();

    // Calculate the bitmask if all the valves, that have flowrates, would be open
    let all_valves_open_bitmask = valves_with_flowrate
        .iter()
        .fold(0, |acc, &id| set_valve_open_state(id, acc, true));

    let max_flow_rate = valve_map.values().map(|v| v.flow_rate).sum();

    Ok(get_max_pressure(
        &mut HashMap::new(),
        &valve_map,
        &valves_with_flowrate,
        steps_left,
        0,
        all_valves_open_bitmask,
        max_flow_rate,
        valve_a_id,
    ))
}

fn part2(input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
