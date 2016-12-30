extern crate neural_nets;

use neural_nets::{vec_mult, NN};

#[test]
fn constructor() {
    let nn = NN::new(4, 3);
    assert!(nn.get_nodes_per_layer() == 3);
    assert!(nn.get_num_layers() == 4);
}

#[test]
fn print_doesnt_panic() {
    let nn = NN::new(3,4);
    nn.print_weights();
}

#[test]
fn vector_multiplication() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![0.5, 0.5, 0.5];
    let c = vec_mult(a,b);
    assert!(c.len() == 3);
    assert!(c[0] == 0.5);
    assert!(c[1] == 1.0);
    assert!(c[2] == 1.5);
}
/*
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
