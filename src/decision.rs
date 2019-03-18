use crate::pattern::Constructor;

#[derive(Debug)]
pub enum DecisionTree {
    /// Success (k is an action ,an Integer)
    Leaf(usize),
    Fail,
    //Failure
    Switch(Vec<(Constructor, DecisionTree)>, Option<Box<DecisionTree>>),
    Swap(usize, Box<DecisionTree>),
}
