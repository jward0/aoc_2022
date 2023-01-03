use regex::Regex;

use std::{
    cmp::{min, max},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

struct State {
    current_location: usize,
    current_flow: u32,
    open_valves: Vec<usize>,
    reward: u32,
    max_reward: u32,
    time: u32,
}

impl State {
    fn step_copy(&self, new_valve: usize, flow_inc: u32, opened_valve: usize, time_inc: u32) -> State {

        let mut open_valves = self.open_valves.to_vec();
        if opened_valve != 0 {
            open_valves.push(opened_valve);
        }
        
        let mut new_state = State{current_location: new_valve,
                                  current_flow: self.current_flow + flow_inc,
                                  open_valves: open_valves,
                                  reward: self.reward + time_inc*self.current_flow,
                                  max_reward: self.max_reward,
                                  time: self.time + time_inc};
        return new_state;
    }
}

fn do_dfs(valve_map: Vec<(usize, u32, Vec<(usize, u32)>)>) -> u32 {

    let max_flow: u32 = valve_map.iter().map(|i| i.1).collect::<Vec<u32>>().iter().sum();

    let state: State = State{current_location: 0, 
                             current_flow: 0,
                             open_valves: vec![0],
                             reward: 0,
                             max_reward: 0,
                             time: 0};

    return step_dfs(&valve_map, max_flow, state);
}

fn step_dfs(valve_map: &Vec<(usize, u32, Vec<(usize, u32)>)>, max_flow: u32, mut state: State) -> u32 {

    if state.time == 30 || state.reward + (30-state.time)*max_flow < state.max_reward {
        return state.reward;
    }

    let current_idx: usize = valve_map.iter().position(|v| v.0 == state.current_location).unwrap();

    for destination in &valve_map[current_idx].2 {
        if destination.0 != state.current_location {

            let target_idx: usize = valve_map.iter().position(|v| v.0 == destination.0).unwrap();
 
            let new_valve = destination.0;
            let mut flow_inc = 0;
            let mut opened_valve = 0;
            let mut time_inc = min(destination.1, 30-state.time);

            if !state.open_valves.contains(&destination.0) && state.time + destination.1 + 1 <= 30 {
                flow_inc = valve_map[target_idx].1;
                opened_valve = destination.0;
                time_inc = destination.1 + 1;
            }
            state.max_reward = max(
                step_dfs(
                    valve_map, 
                    max_flow, 
                    state.step_copy(new_valve, flow_inc, opened_valve, time_inc)), 
                state.max_reward);
        }
    }
    return state.max_reward;
}


fn part_1<R: Read>(io: R) -> Result<u32, Error> {

    let mut valves_map: [(u32, Vec<usize>, u32, bool); 676] = [(); 676].map(|_| (0, Vec::new(), 0, false)); // [(0, vec![]); 676];


    let valves_re = Regex::new(r"([A-Z]{2})").unwrap();
    let flow_re = Regex::new(r"(\d+)").unwrap();

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = &line.unwrap();
        let flow_rate = flow_re.captures(l).unwrap()[1].parse::<u32>().unwrap();
        let valve_captures = valves_re.find_iter(l).map(|i| i.as_str()).collect::<Vec<&str>>();
        let adjacent_valves: Vec<&str> = valve_captures[1..].iter().map(|i| *i).collect::<Vec<&str>>();

        let self_idx: usize = (valve_captures[0].chars().nth(0).unwrap() as usize - 65)*26 + valve_captures[0].chars().nth(1).unwrap() as usize - 65;

        let neighbours_idx: Vec<usize> = adjacent_valves.iter().map(
            |m| (m.chars().nth(0).unwrap() as usize - 65)*26 + m.chars().nth(1).unwrap() as usize - 65)
            .collect::<Vec<usize>>();

        valves_map[self_idx].0 = flow_rate;
        valves_map[self_idx].1 = neighbours_idx;
    }

    let mut distances: [[u32; 676]; 676] = [[1000; 676]; 676];

    for (i, v) in valves_map.iter().enumerate() {
        for j in &v.1 {
            distances[i][*j] = 1;
        }
        distances[i][i] = 0;
    }

    for k in 0..valves_map.len() -1 {
        for i in 0..valves_map.len() -1 {
            for j in 0..valves_map.len() - 1 {
                if distances[i][j] > distances[i][k] + distances[k][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }

    let mut relevant_valves: Vec<usize> = vec![];
    relevant_valves.push(0);
    for (i, valve) in valves_map.iter().enumerate() {
        if valve.0 > 0 {
            relevant_valves.push(i);
        }
    }
    // Index, flow rate, Vec<(index, distance to)>
    let mut relevant_map: Vec<(usize, u32, Vec<(usize, u32)>)> = vec![];

    for i in &relevant_valves {
        let mut distances_to_relevant: Vec<(usize, u32)> = vec![];
        for j in &relevant_valves {
            if i != j {
                distances_to_relevant.push((*j, distances[*i][*j]));
            }
        }
        relevant_map.push((*i, valves_map[*i].0, distances_to_relevant));
    }
    // println!("{:?}", relevant_map);
    Ok(do_dfs(relevant_map))
}

fn main() {
    println!("{:?}", part_1(File::open("input.txt").unwrap()));
}
