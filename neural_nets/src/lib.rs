/// Basic Feed Forward Neural Network
extern crate rand;
use rand::Rng;


pub struct NN {
    nodes_per_layer: usize,
    num_layers: usize,
    weights: Vec<Vec<f64>>,
}

impl NN {
    pub fn new(layers: usize, nodes: usize) -> NN {
        // weights are initialized to 0.0
        NN {
            nodes_per_layer: nodes,
            num_layers: layers,
            weights:  vec![ vec![0.0;nodes] ; layers+1 ],
        }
    }

    pub fn get_nodes_per_layer(&self) -> usize{
        self.nodes_per_layer
    }

    pub fn get_num_layers(&self) -> usize {
        self.num_layers
    }

    pub fn get_weights(&self) -> Vec<Vec<f64>> {
        self.weights.to_vec()
    }

    pub fn print_weights(&self) {
        let weight_layers = self.num_layers + 1;
        let nodes = self.nodes_per_layer;
        for l in 0..weight_layers {
            for n in 0..nodes {
                print!("{} ", self.weights[l][n]);
            }
            print!("\n");
        }
    }

    pub fn randomize_weights(&mut self) {
        for l in 0..self.num_layers+1 {
            for n in 0..self.nodes_per_layer {
                self.weights[l][n] = rand::thread_rng().gen_range(-1.0, 1.0);
            }
        }
    }

    pub fn sigmoid(x: f64) -> f64 {
        1f64 / (1f64 + (-x).exp())
    }

    pub fn evaluate(input: Vec<f64>) -> f64 {
        // TODO, how to check that the input's size is correct?
        // maybe just return an Option<Vec<f64>>?
        0.0
    }

}

pub fn vec_mult(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
        // TODO use a map?
        let mut ret = Vec::new();
        for i in 0..a.len(){
            ret.push(a[i]*b[i]);
        }
        ret
}


/*
fn main() {
    let mut nn = NN::new(3, 3);
    nn.print_weights();
    nn.randomize_weights();
    nn.print_weights();
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![0.5, 0.5, 0.5];
    let c = vec_mult(a,b);
    println!("a*b = ");
    for e in c.iter(){
        println!("{}",e);
    }
}
*/
