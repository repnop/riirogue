//! The module for pathing stuff.

#![allow(unused_mut)]

use std::collections::HashMap;
use std::collections::HashSet;

/// Computes a path with A* using only orthographic movement.
///
/// You must provide a function that determines if a location is walkable, and
/// also another function that gives the move cost to go from A to B. The second
/// function will never be called if the first function declares B to not be
/// walkable, if that helps you somehow.
pub fn ortho_star<W, MC>(
    start: (i32, i32),
    goal: (i32, i32),
    walkable: W,
    move_cost: MC,
) -> Option<Vec<(i32, i32)>>
where
    W: Fn(i32, i32) -> bool,
    MC: Fn((i32, i32), (i32, i32)) -> u32,
{
    if !walkable(goal.0, goal.1) {
        return None;
    } else {
        // default heuristic is the manhattan dist
        let heuristic = |start: (i32, i32), goal: (i32, i32)| {
            (start.0 - goal.0).abs() as u32 + (start.1 - goal.1).abs() as u32
        };
        let mut closed_set: HashSet<(i32, i32)> = HashSet::new();
        let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut open_set: HashSet<(i32, i32)> = HashSet::new();
        open_set.insert(start);
        let mut g_score: HashMap<(i32, i32), u32> = HashMap::new();
        g_score.insert(start, 0u32);
        let mut f_score: HashMap<(i32, i32), u32> = HashMap::new();
        f_score.insert(start, heuristic(start, goal));
        while !open_set.is_empty() {
            let current = *open_set
                .iter()
                .min_by_key(|loc_ref| f_score[loc_ref])
                .expect("the open set can never be empty here!");
            if current == goal {
                return Some(reconstruct_path(came_from, current));
            } else {
                open_set.remove(&current);
                closed_set.insert(current);
                for &neighbor in [
                    (current.0 + 1, current.1),
                    (current.0 - 1, current.1),
                    (current.0, current.1 + 1),
                    (current.0, current.1 - 1),
                ].iter()
                    .filter(|(x, y)| walkable(*x, *y) && !closed_set.contains(&(*x, *y)))
                {
                    open_set.insert(neighbor);
                    let tentative_g_score =
                        g_score[&current].saturating_add(move_cost(current, neighbor));
                    if tentative_g_score >= *g_score.entry(neighbor).or_insert(::std::u32::MAX) {
                        continue;
                    } else {
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);
                        f_score.insert(neighbor, g_score[&neighbor] + heuristic(neighbor, goal));
                    }
                }
            }
        }
        None
    }
}

fn reconstruct_path(
    came_from: HashMap<(i32, i32), (i32, i32)>,
    mut current: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut total_path = vec![current];
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(current);
    }
    total_path
}
