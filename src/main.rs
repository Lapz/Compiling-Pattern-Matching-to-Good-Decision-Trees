use crate::decision::DecisionTree;
use crate::matrix::{PatternMatrix, Row};
use crate::pattern::{list, split, wcard};
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::hint::unreachable_unchecked;

mod decision;
mod matrix;
mod pattern;

/*

    let mut matrix = PatternMatrix::new();

    matrix.add_row(Row::new(vec![nil(),Pattern::WildCard]));
    matrix.add_row(Row::new(vec![Pattern::WildCard,nil()]));

    ( Nil, _  )
    ( _  ,Nil )
*/

fn compile_patterns(occurrences: &mut Vec<()>, matrix: &mut PatternMatrix) -> DecisionTree {
    if matrix.is_empty() {
        return DecisionTree::Fail;
    } else if matrix.get(0).contains_wcard_only() {
        //check if the first row only contains wildcards
        DecisionTree::Leaf(matrix.get(0).action())
    } else {
        let columns = matrix.cols_with_con(); //All columns that have no wildcard pattern;

        let i = columns[0];

        if  i == 0 {
            let head_cons = matrix.head_cons();
            let mut default = None;
            let mut case_list = Vec::new();

            if head_cons.is_empty() {
                default = Some(Box::new(compile_patterns(
                    occurrences,
                    &mut matrix.default(),
                )));
            }

            for con in head_cons {
                let mut matrix = matrix.specialization(&con);

                case_list.push((con.clone(), compile_patterns(occurrences, &mut matrix)));
            }

            DecisionTree::Switch(case_list, default)
        } else {
            matrix.swap(i);
            DecisionTree::Swap(i, Box::new(compile_patterns(occurrences, matrix)))
        }
    }
}

fn main() {
    let mut matrix = PatternMatrix::new();

    //matrix P -> A
    matrix.add_row(Row::new(vec![list(vec![]), wcard()], 1));
    matrix.add_row(Row::new(vec![wcard(), list(vec![])], 2));
    matrix.add_row(Row::new(
        vec![split(wcard(), wcard()), split(wcard(), wcard())],
        3,
    ));;

    println!("{}", matrix);

    println!("{}", compile_patterns(&mut vec![], &mut matrix));

    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use crate::matrix::{PatternMatrix, Row};
    use crate::pattern::{list, split, wcard, Constructor};

    #[test]
    fn specialization_on_split_works() {
        let mut matrix = PatternMatrix::new();

        //matrix P -> A
        matrix.add_row(Row::new(vec![list(vec![]), wcard()], 1));
        matrix.add_row(Row::new(vec![wcard(), list(vec![])], 2));
        matrix.add_row(Row::new(
            vec![split(wcard(), wcard()), split(wcard(), wcard())],
            3,
        ));

        ;

        let mut expected = PatternMatrix::new();

        expected.add_row(Row::new(vec![wcard(), wcard(), list(vec![])], 2));
        expected.add_row(Row::new(vec![wcard(), wcard(), split(wcard(), wcard())], 3));

        assert_eq!(
            matrix.specialization(&Constructor {
                name: "Split".into(),
                arity: 2,
                span: 1,
            }),
            expected
        )
    }

    #[test]
    fn specialization_on_list_works() {
        let mut matrix = PatternMatrix::new();

        //matrix P -> A
        matrix.add_row(Row::new(vec![list(vec![]), wcard()], 1));
        matrix.add_row(Row::new(vec![wcard(), list(vec![])], 2));
        matrix.add_row(Row::new(
            vec![split(wcard(), wcard()), split(wcard(), wcard())],
            3,
        ));

       ;

        let mut expected = PatternMatrix::new();

        expected.add_row(Row::new(vec![wcard()], 1));
        expected.add_row(Row::new(vec![wcard(), wcard(), list(vec![])], 2));

        assert_eq!(
            matrix.specialization(&Constructor {
                name: "List".into(),
                arity: 0,
                span: 1,
            }),
            expected
        )
    }

    #[test]
    fn default_works() {
        let mut matrix = PatternMatrix::new();

        //matrix P -> A
        matrix.add_row(Row::new(vec![list(vec![]), wcard()], 1));
        matrix.add_row(Row::new(vec![wcard(), list(vec![])], 2));
        matrix.add_row(Row::new(vec![wcard(), wcard()], 3));

        let mut expected = PatternMatrix::new();

        expected.add_row(Row::new(vec![list(vec![])], 2));
        expected.add_row(Row::new(vec![wcard()], 3));

        assert_eq!(matrix.default(), expected)
    }
}
