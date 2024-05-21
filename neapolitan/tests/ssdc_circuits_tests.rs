use neapolitan::ssdc_circuits::{SSDCCircuit, resistor, voltage_source};

#[test]
fn architecture_test()
{
    let mut circuit = SSDCCircuit::new();
    circuit.add_nodes(4);
    circuit.ground_node(0);

    //                  Element         Output  Input   Voltage/Resistance
    circuit.add_element(voltage_source, 0,      1,      3.0).expect("Failed to make voltage source"); 
    circuit.add_element(resistor,       1,      2,      2.0).expect("Failed to make 2 ohm resistor");
    circuit.add_element(resistor,       2,      3,      1.0).expect("Failed to make first 1 ohm resistor");
    circuit.add_element(resistor,       3,      0,      1.0).expect("Failed to make second 1 ohm resistor");

    let soln = circuit.solve()
        .expect("Failed to solve model");

    println!("{:#?}", soln);
}