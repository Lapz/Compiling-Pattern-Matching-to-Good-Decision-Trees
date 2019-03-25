use crate::pattern::Constructor;
use std::fmt::{self, Display, Write};

#[derive(Debug)]
pub enum DecisionTree {
    /// Success (k is an action ,an Integer)
    Leaf(usize),
    Fail,
    //Failure
    Switch(Vec<(Constructor, DecisionTree)>, Option<Box<DecisionTree>>),
    Swap(usize, Box<DecisionTree>),
}


impl Display for DecisionTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecisionTree::Leaf(ref size) => write!(f, "Leaf({})", size),
            DecisionTree::Fail => write!(f,"Fail"),
            DecisionTree::Switch(ref stack, ref default) => {
                write!(f, "Switch(")?;

                for (i, (con, tree)) in stack.iter().enumerate() {
                    if i + 1 == stack.len() {
                        write!(f, " ({}):{} ", con, tree)?;
                    } else {
                        write!(f, " ({}):{}, ", con, tree)?;
                    }
                }

                if let Some(ref default) = default {
                    write!(f, " default : {} ", default)?;
                }

                write!(f, ")")
            }
            DecisionTree::Swap(ref size, ref inner) => write!(f, "Swap^{}({})", size, inner),
        }
    }
}
