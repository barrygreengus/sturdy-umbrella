/// Basic Feed Forward Neural Network
//use std::io;
extern crate rand;
use rand::Rng;

struct NN {
    nodes_per_layer: usize,
    num_layers: usize,
    weights: Vec<Vec<f64>>,
}

impl NN {
    fn new(layers: usize, nodes: usize) -> NN {
        // weights are initialized to 0.0
        NN {
            nodes_per_layer: layers,
            num_layers: nodes,
            weights:  vec![ vec![0.0;nodes] ; layers+1 ],
        }
    }

    fn print_weights(&self) {
        let weight_layers = self.num_layers + 1;
        let nodes = self.nodes_per_layer;
        for l in 0..weight_layers {
            for n in 0..nodes {
                print!("{} ", self.weights[l][n]);
            }
            print!("\n");
        }
    }

    fn randomize_weights(&mut self) {
        for l in 0..self.num_layers+1 {
            for n in 0..self.nodes_per_layer {
                self.weights[l][n] = rand::thread_rng().gen_range(-1.0, 1.0);
            }
        }
    }

}

fn main() {
    let mut nn = NN::new(3, 3);
    nn.print_weights();
    nn.randomize_weights();
    nn.print_weights();

}

