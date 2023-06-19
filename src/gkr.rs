use std::marker::PhantomData;

use ark_ff::PrimeField;

pub struct Wiring {
    curr_index: usize,
    left: usize,
    right: usize,
}

impl Wiring {
    fn new(curr_index: usize, left: usize, right: usize) -> Self {
        Self{curr_index, left, right}
    }
}

pub struct Layer {
    // index: u64,
    pub add: Vec<Wiring>,
    pub mul: Vec<Wiring>,
}

impl Layer {
    fn new(add: Vec<Wiring>, mul: Vec<Wiring>) -> Self {
        Self{add, mul}
    }
}

pub struct UniformCircuit<F> {
    layers: Vec<Layer>,
    phantom: PhantomData<F>,
}

impl<F: PrimeField> UniformCircuit<F> {
    fn new(layers: Vec<Layer>) -> Self {
        Self{layers, phantom: PhantomData::<F>}
    }

    fn evaluate(&self, x: Vec<F>) -> Vec<F> {

        let mut last_layer = x;

        for layer in self.layers.iter().rev() {
            let mut new_layer: Vec<F> = vec![F::zero(); layer.add.len() + layer.mul.len()];

            // handle addition
            for Wiring{curr_index, left, right} in layer.add {
                new_layer[curr_index] = last_layer[left] + last_layer[right];
            }
            
            last_layer = new_layer;
        }

        last_layer

    }
}

/* Sanity checks
    - wiring indices in correct range
*/

mod test {
    
    use super::*;

    #[test]
    fn simple_circuit() {
        // mul, layer 0
        let mul0_0 = Wiring::new(0, 0, 1);
        let mul0_1 = Wiring::new(1, 2, 3);
        
        let layer_0 = Layer::new(Vec::new(), vec![mul0_0, mul0_1]);

        // add, layer 0
        // empty

        // mul, layer 1
        let mul1_0 = Wiring::new(0, 0, 0);
        let mul1_1 = Wiring::new(1, 1, 1);
        let mul1_2 = Wiring::new(2, 1, 2);
        
        // add, layer 1
        let add1_3 = Wiring::new(3, 3, 3);
        
        let layer_1 = Layer::new(vec![add1_3], vec![mul1_0, mul1_1, mul1_2]);

        let circuit = UniformCircuit::new(vec![layer_0, layer_1]);
    }
}