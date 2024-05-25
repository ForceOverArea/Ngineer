use gmatlib::col_vec;
use neapolitan::{get_node_potential, set_node_potential, ssdc_circuits::{resistor, voltage_source, SSDCCircuit}, GenericNode};
use rand::random;
use std::rc::Rc;
use gmatlib::Matrix;

#[test]
fn architecture_test()
{
    let mut circuit = SSDCCircuit::new();
    circuit.add_nodes(4);
    circuit.ground_node(0);

    //                  Element         Input   Output  Voltage/Resistance
    circuit.add_element(voltage_source, 0,      1,      3.0).expect("Failed to make voltage source"); 
    circuit.add_element(resistor,       1,      2,      2.0).expect("Failed to make 2 ohm resistor");
    circuit.add_element(resistor,       2,      3,      1.0).expect("Failed to make first 1 ohm resistor");
    circuit.add_element(resistor,       3,      0,      1.0).expect("Failed to make second 1 ohm resistor");

    let soln = circuit.solve()
        .expect("Failed to solve model");

    println!("{:#?}", soln);
}

#[test]
fn fuzz_resistor_flux_calcs()
{
    for _ in 0..1000
    {
        let test_res = random::<f64>();
        let output_potential = random::<f64>();

        let node1 = GenericNode::new();
        let node2 = GenericNode::new();

        set_node_potential(&Rc::downgrade(&node2), col_vec![output_potential]).unwrap();
        let res = resistor(Rc::downgrade(&node1), Rc::downgrade(&node2), test_res).unwrap();

        let expected = (1.0 - output_potential) / test_res;

        assert!(output_potential
                - get_node_potential(&Rc::downgrade(&node2)).unwrap()[(0, 0)]
                < 1E-10);

        // println!("input_pot:  {}", get_node_potential(&Rc::downgrade(&node1)).unwrap());
        // println!("output_pot: {}", get_node_potential(&Rc::downgrade(&node2)).unwrap());
        // println!("resistance: {test_res}");
        // println!("expected:   {expected}");

        assert!(expected 
                - res.get_flux().unwrap()[(0, 0)]
                < 1E-10);
    }
}