use std::fmt::{self, Display};
use crate::matrix::{PatternMatrix, Row};
use crate::pattern::{wcard, list};

mod matrix;
mod pattern;

/*

    let mut matrix = PatternMatrix::new();

    matrix.add_row(Row::new(vec![nil(),Pattern::WildCard]));
    matrix.add_row(Row::new(vec![Pattern::WildCard,nil()]));

    ( Nil, _  )
    ( _  ,Nil )
*/

fn main() {
    let mut matrix = PatternMatrix::new();

    //matrix Q -> B
    matrix.add_row(Row::new(vec![list(vec![]), wcard()]));
    matrix.add_row(Row::new(vec![wcard(), list(vec![])]));
    matrix.add_row(Row::new(vec![wcard(), wcard()]));

    println!("{}",matrix.default());

    println!("Hello, world!");
}





#[cfg(test)]
mod test {

    use crate::matrix::{PatternMatrix, Row};
    use crate::pattern::{split, wcard, list, Constructor};

    #[test]
    fn specialization_on_split_works() {
        let mut matrix = PatternMatrix::new();

        //matrix P -> A
        matrix.add_row(Row::new(vec![list(vec![]), wcard()]));
        matrix.add_row(Row::new(vec![wcard(), list(vec![])]));
        matrix.add_row(Row::new(vec![
            split(wcard(), wcard()),
            split(wcard(), wcard()),
        ]));

        ;

        let mut expected = PatternMatrix::new();

        expected.add_row(Row::new(vec![wcard(), wcard(), list(vec![])]));
        expected.add_row(Row::new(vec![wcard(), wcard(), split(wcard(), wcard())]));

        assert_eq!(matrix.specialization(&Constructor {
            name: "Split".into(),
            arity: 2,
            span: 1,
        }), expected)
    }

    #[test]
    fn specialization_on_list_works() {
        let mut matrix = PatternMatrix::new();

        //matrix P -> A
        matrix.add_row(Row::new(vec![list(vec![]), wcard()]));
        matrix.add_row(Row::new(vec![wcard(), list(vec![])]));
        matrix.add_row(Row::new(vec![
            split(wcard(), wcard()),
            split(wcard(), wcard()),
        ]));

       ;

        let mut expected = PatternMatrix::new();

        expected.add_row(Row::new(vec![wcard()]));
        expected.add_row(Row::new(vec![wcard(), wcard(), list(vec![])]));

        assert_eq!( matrix.specialization(&Constructor {
            name: "List".into(),
            arity: 0,
            span: 1,
        }), expected)
    }

    #[test]
    fn default_works() {
        let mut matrix = PatternMatrix::new();

        //matrix P -> A
        matrix.add_row(Row::new(vec![list(vec![]), wcard()]));
        matrix.add_row(Row::new(vec![wcard(), list(vec![])]));
        matrix.add_row(Row::new(vec![wcard(), wcard()]));

        matrix.default();

        let mut expected = PatternMatrix::new();

        expected.add_row(Row::new(vec![list(vec![])]));
        expected.add_row(Row::new(vec![wcard()]));

        assert_eq!(matrix, expected)
    }
}
