use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::Matrix;
use crate::errors::{DroppedNodeError, FluxCalculationError};
use crate::modelling::node::GenericNode;

/// The function signature for a `neapolitan`-compatible element constructor function.
/// 
/// # Design Philosophy:
/// Because Rust does not support inheritance of shared data between types, 
/// extending `neapolitan` requires that authors write a constructor *function* rather than a method that 
/// packs the relevant data into a `GenericElement` for use by the solver engine. This 
/// design pattern ensures that when an user needs to add a new variety of element for a model, it has all the 
/// data required by the solver engine to properly solve the model. 
/// 
/// # Argument Types:
/// The type `T` is left as a parameter to allow authors to control what type of data is 
/// used in deriving the `gain` value as well as additional metadata used in flux calculations. For example, in 
/// steady-state DC circuitry, `T` is simply `f64` to represent a real value for the voltage,
/// resistance, or current of the element being created. In AC circuitry problems,
/// however, this type would be `(f64, f64)` to represent the real and imaginary components of
/// current at that location.
/// 
/// Constructor functions operate on `Weak<RefCell<GenericNode>>` smart pointers in order to
/// properly create the network structure used to model problems. These node references must
/// be passed on to the created element, but may be operated on prior to element creation. 
/// E.g. `voltage_source` sets a driven node's voltage to the driving node's potential +/- the
/// potential difference specified prior to connecting the element to the network. 
pub type ElementConstructor = fn (Weak<RefCell<GenericNode>>, Weak<RefCell<GenericNode>>, Vec<f64>) -> anyhow::Result<Rc<GenericElement>>;

/// The function signature used to calculate flux between nodes.
/// 
/// # Design Philosophy
/// Flux calculation functions are the other function that must be defined in order to extend 
/// `neapolitan`. They are implemented as a single function pointer rather than a trait object
/// for simplicity. This ensures that implementors do not have to define a type that does not 
/// cleanly mimic a `GenericElement` just to implement a common method.
/// 
/// These functions get called by the `GenericElement::get_flux` method. They must 
/// return a `Matrix<f64>` containing the flux value that the element should have based on its 
/// nodal potentials and `gain` value. Similarly to `ElementConstructor<T>`, this function type 
/// must work with smart pointers to `GenericNode`s, but this time must work with `Rc<RefCell<GenericNode>>` 
/// instead of `Weak<RefCell<GenericNode>>` as the type. They additionally take a `&Matrix<f64>` 
/// argument that will be equivalent to the element's `gain` value and a `bool` that indicates 
/// the directionality of some quantity. This value is fairly arbitrary and it is up to the 
/// implementor to choose its meaning.
/// 
/// # Note on Runtime Borrowing:
/// Upon being called, these functions may have more or less whatever side effects they need to in
/// order to make their calculations work. One key thing to keep in mind however, is that elements
/// that require calculating a node's flux discrepancy value in order to determine a flux must **not**
/// mutably borrow that node or any other node used in the flux discrepancy calculation to avoid
/// raising an `AlreadyMutablyBorrowed` error.  
pub type FluxCalculation = fn (Rc<RefCell<GenericNode>>, Rc<RefCell<GenericNode>>, &Matrix<f64>, bool) -> anyhow::Result<Matrix<f64>>;

/// A struct representing an element in a nodal analysis problem.
/// 
/// # Concept:
/// Elements are conductors for some "flux" value between nodes in a 
/// nodal analysis problem. In DC circuits models, resistors, voltage 
/// sources, and current sources are examples of elements as they 
/// allow current (i.e. flux) to "travel" between two nodes. The way in
/// which this flux value is calculated may vary wildly between elements,
/// but **must** operate only on knowing which nodes they connect, their
/// own gain value (e.g. resistance, voltage, and current, respectively in this case), 
/// and which node's potential they drive, if they drive one at all.
#[derive(Clone, Debug)]
pub struct GenericElement
{
    gain: Matrix<f64>,
    input_node: Weak<RefCell<GenericNode>>,
    output_node: Weak<RefCell<GenericNode>>,
    flux_calc: FluxCalculation,
    drives_output: bool,
}
impl GenericElement
{
    /// Attempts to construct a new `GenericElement`, possibly returning a `DroppedNodeError`
    /// if the nodes being connected were previously dropped for some reason. This method is
    /// intended for use in an `ElementConstructor<T>`-compatible function.
    /// 
    /// # Arguments of Interest
    /// `flux_calc` - a function pointer to the flux calculation that this element should perform.
    /// 
    /// `drives_output` - an arbitrary value used to indicate directionality. For example, voltage
    /// source elements in DC circuitry problems use this to determine whether they should control
    /// the input or output node's potential value.
    /// 
    /// `connect_to_input` and `connect_to_output` - boolean flags that indicate whether the input 
    /// and output nodes, respectively, should be made aware of the element. Usually these will 
    /// always be true, but elements that use an adjacent node's flux balance in their flux 
    /// calculation may need to intentionally not connect to that element. If they do, they will 
    /// recursively call `GenericElement::get_flux` and cause a stack overflow unless some other 
    /// action is taken.
    /// 
    /// # Example
    /// ```
    /// use std::rc::{Rc, Weak};
    /// use std::cell::RefCell;
    /// use neapolitan::modelling::{GenericElement, GenericNode};
    /// use neapolitan::flux_formulas::constant_flux;
    /// use neapolitan::errors::DroppedNodeError;
    /// 
    /// pub fn current_source(
    ///     input: Weak<RefCell<GenericNode>>, 
    ///     output: Weak<RefCell<GenericNode>>, 
    ///     current: f64
    /// ) -> Result<Rc<GenericElement>, Box<dyn std::error::Error>>
    /// {
    ///     Ok(GenericElement::try_new(
    ///         vec![current],
    ///         input, output,
    ///         constant_flux,
    ///         false,      // We don't need this information. Just make it `false` 
    ///         true, true, // Connect the input and output to the element.
    ///     )?)
    /// }
    /// ```
    pub fn try_new(gain: Vec<f64>,
        input_node: Weak<RefCell<GenericNode>>,
        output_node: Weak<RefCell<GenericNode>>,
        flux_calc: FluxCalculation,
        drives_output: bool,
        connect_to_input: bool,
        connect_to_output: bool,
    ) -> anyhow::Result<Rc<GenericElement>>
    {
        let elem = Rc::new(
            GenericElement 
            {
                gain: Matrix::from_col_vec(gain), 
                input_node: Weak::clone(&input_node), 
                output_node: Weak::clone(&output_node), 
                flux_calc, 
                drives_output 
            }
        );

        // Make nodes aware of element
        if let (Some(input_refcell), Some(output_refcell)) = (input_node.upgrade(), output_node.upgrade())
        {
            if connect_to_input
            {
                input_refcell.try_borrow_mut()?
                    .outputs.push(Rc::clone(&elem));
            }

            if connect_to_output
            {
                output_refcell.try_borrow_mut()?
                    .inputs.push(Rc::clone(&elem));
            }

            Ok(elem)
        }
        else 
        {
            Err(DroppedNodeError.into())
        }
    }

    /// Calculates the flux for this element by calling the `FluxCalculation` function pointer
    /// it owns.
    /// 
    /// # Example
    /// ```
    /// use std::rc::Rc;
    /// use neapolitan::set_node_potential;
    /// use neapolitan::modelling::{GenericElement, GenericNode};
    /// use neapolitan::ssdc_circuits::resistor;
    /// 
    /// let a = GenericNode::new();
    /// let b = GenericNode::new();
    /// 
    /// set_node_potential(&Rc::downgrade(&a), vec![3.0]).unwrap();
    /// 
    /// let elem = resistor(Rc::downgrade(&a), Rc::downgrade(&b), vec![2.0]).unwrap();
    /// let flux = Vec::from(
    ///     elem.get_flux().unwrap()
    /// );
    /// 
    /// assert_eq!(vec![1.0], flux);
    /// ```
    pub fn get_flux(&self) -> anyhow::Result<Matrix<f64>>
    {
        if let (Some(inode), Some(onode)) = (self.input_node.upgrade(), self.output_node.upgrade())
        {
            Ok((self.flux_calc)(inode, onode, &self.gain, self.drives_output)?)
        }
        else
        {
            Err(FluxCalculationError::NodeRefsAlreadyDropped.into())
        }
    }
}