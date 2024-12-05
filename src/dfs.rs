use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub enum DfsError {
    CycleDetected
}

fn visit<'a, N, E : Iterator<Item = &'a N>>(
    node: &'a N,
    edge_map: &impl Fn(&'a N)->E,
    on_visit: &mut impl FnMut(&'a N),
    unvisited: &mut BTreeSet<&'a N>,
    in_progress: &mut HashSet<&'a N>,
) -> Result<(), DfsError> where N: Ord + Hash {
    if !unvisited.contains(node) {
        return Ok(());
    }
    if in_progress.contains(node) {
        return Err(DfsError::CycleDetected);
    }

    in_progress.insert(node);

    for next_page in edge_map(node) {
        match visit(next_page, edge_map, on_visit, unvisited, in_progress) {
            Ok(_) => {}, // Ignore
            Err(e) => { return Err(e); },
        }
    }

    in_progress.remove(node);
    unvisited.remove(node);
    on_visit(node);

    Ok(())
}

pub fn depth_first_search<'a, N, E : Iterator<Item = &'a N>>(
    nodes: &'a [N],
    edge_map: impl Fn(&'a N)->E,
    mut on_visited: impl FnMut(&'a N),
) -> Result<(), DfsError> where N: Ord + Eq + Hash{
    let mut unvisited = BTreeSet::from_iter(nodes);
    let mut in_progress = HashSet::new();

    while let Some(node) = unvisited.first() {
        match visit(*node, &edge_map, &mut on_visited, &mut unvisited, &mut in_progress) {
            Ok(_) => {}, // Ignore
            Err(e) => { return Err(e); },
        }
    }

    Ok(())
}
