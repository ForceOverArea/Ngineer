use std::rc::Rc;

use rand::random;

use neapolitan::{get_node_potential, set_node_potential, NodalAnalysisStudyBuilder};
use neapolitan::dc_circuits::{resistor, RESISTOR, DC_CIRCUIT, VOLTAGE_SOURCE};
use neapolitan::modelling::node::GenericNode;


#[test]
fn architecture_test()
{
    let mut save = String::new();
    let builder = NodalAnalysisStudyBuilder::new(DC_CIRCUIT.to_string(), None)
        .expect("failed to create model builder object");
    // Add nodes to system
    let soln = builder
        // Add nodes:
        .add_nodes(4)
        // Ground node 0:
        .configure_node(0, vec![0.0], true, None)
        // Add elements:
        //           Element         Input   Output  Gain
        .add_element(VOLTAGE_SOURCE, 0,      1,      vec![3.0]).expect("Failed to make voltage source") 
        .add_element(RESISTOR,       1,      2,      vec![2.0]).expect("Failed to make 2 ohm resistor")
        .add_element(RESISTOR,       2,      3,      vec![1.0]).expect("Failed to make first 1 ohm resistor")
        .add_element(RESISTOR,       3,      0,      vec![1.0]).expect("Failed to make second 1 ohm resistor")
        // Save model as a JSON string
        .save_model(&mut save).expect("Failed to save model")
        // Solve the model:
        .run_study(0.0001, 100).expect("Failed to solve model");

    println!("{:#?}", save);
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

        set_node_potential(&Rc::downgrade(&node2), vec![output_potential]).unwrap();
        let res = resistor(Rc::downgrade(&node1), Rc::downgrade(&node2), vec![test_res]).unwrap();

        let expected = (1.0 - output_potential) / test_res;

        assert!(output_potential - get_node_potential(&Rc::downgrade(&node2)).unwrap()[(0, 0)] < 1E-10);

        assert!(expected - res.get_flux().unwrap()[(0, 0)] < 1E-10);
    }
}