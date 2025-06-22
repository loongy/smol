use std::collections::HashMap;

use crate::graph::{BondOrder, Graph};

pub fn parse_smiles(input: &str) -> Graph {
    let mut elements: Vec<String> = Vec::new();
    let mut edges: Vec<(usize, usize, BondOrder)> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut ring_map: HashMap<u32, (usize, BondOrder)> = HashMap::new();

    let mut chars = input.chars().peekable();
    let mut last_atom: Option<usize> = None;
    let mut pending_bond = BondOrder::Single;

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                if let Some(idx) = last_atom {
                    stack.push(idx);
                }
            }
            ')' => {
                last_atom = stack.pop();
            }
            '-' => pending_bond = BondOrder::Single,
            '=' => pending_bond = BondOrder::Double,
            '#' => pending_bond = BondOrder::Triple,
            '1'..='9' => {
                let digit = c.to_digit(10).unwrap();
                if let Some((other, bond)) = ring_map.remove(&digit) {
                    let src = last_atom.expect("ring digit must follow an atom");
                    edges.push((src, other, bond));
                    edges.push((other, src, bond));
                } else {
                    let idx = last_atom.expect("ring digit must follow an atom");
                    ring_map.insert(digit, (idx, pending_bond));
                }
                pending_bond = BondOrder::Single;
            }
            'A'..='Z' => {
                let mut element = String::new();
                element.push(c);
                if let Some(next) = chars.peek() {
                    if next.is_ascii_lowercase() {
                        element.push(chars.next().unwrap());
                    }
                }
                let idx = elements.len();
                elements.push(element);
                if let Some(prev) = last_atom {
                    edges.push((prev, idx, pending_bond));
                    edges.push((idx, prev, pending_bond));
                }
                last_atom = Some(idx);
                pending_bond = BondOrder::Single;
            }
            _ => {
                // ignore unsupported characters
            }
        }
    }

    let num_atoms = elements.len();
    Graph::new(num_atoms, edges, elements)
}
