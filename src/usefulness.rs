use crate::decision::DecisionTree;
use crate::matrix::{PatternMatrix, Row};
use crate::pattern::{list, nil, split, wcard, Constructor, Pattern};
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::hint::unreachable_unchecked;

pub fn usefulness(mut matrix: PatternMatrix, mut row: PatternMatrix) -> bool {
    if matrix.is_empty() {
        true
    } else {
        if row.get(0)[0].is_con() {
            let con = row.get(0)[0].con().unwrap();

            usefulness(matrix.specialization(&con), row.specialization(&con))
        } else {
            unimplemented!()
        }
    }
}
