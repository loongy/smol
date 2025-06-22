mod graph;
mod parser;

use graph::BondOrder;
use parser::parse_smiles;

fn main() {
    let smiles = "C1=CC=CC=C1"; // benzene
    let graph = parse_smiles(smiles);
    println!("Parsed {} atoms", graph.elements.len());
    for i in 0..graph.elements.len() {
        print!("Atom {} ({}):", i, graph.elements[i]);
        for (nbr, order) in graph.neighbors(i) {
            let order_str = match order {
                BondOrder::Single => "-",
                BondOrder::Double => "=",
                BondOrder::Triple => "#",
            };
            print!(" {}{}", order_str, nbr);
        }
        println!();
    }
}
