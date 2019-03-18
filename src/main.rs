use std::fmt::{self, Display};
use std::marker::PhantomData;
/// A pattern Constructor
/// i.e given enum RGB {
///     Red,
///     Green,
///     Blue
/// }
///
/// Red => Constructor {
///     name:"Red",
///     arity:0,
///     span:3,
/// }
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Constructor {
    name: String,
    arity: i32,
    span: i32,
}

/// A pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Pattern {
    //    Var(String),                    // e.g foo /vars
    Con(Constructor, Vec<Pattern>), //e.g. List(10,Nil)
    WildCard,
    Or(Box<Pattern>, Box<Pattern>),
}

fn nil() -> Pattern {
    Pattern::Con(
        Constructor {
            name: "Nil".into(),
            arity: 0,
            span: 1,
        },
        vec![],
    )
}

/// list split  pattern / x ::xy
fn split(lhs: Pattern, rhs: Pattern) -> Pattern {
    let con = Constructor {
        name: "Split".into(),
        arity: 2,
        span: 1,
    };

    Pattern::Con(con, vec![lhs, rhs])
}

fn list(args: Vec<Pattern>) -> Pattern {
    let con = Constructor {
        name: "List".into(),
        arity: args.len() as i32,
        span: 1,
    };

    Pattern::Con(con, args)
}

fn wcard() -> Pattern {
    Pattern::WildCard
}

#[derive(Debug, PartialEq)]
struct Row(Vec<Pattern>);

impl Row {
    pub fn new(pats: Vec<Pattern>) -> Self {
        Self(pats)
    }

    pub fn add(&mut self, pat: Pattern) {
        self.0.push(pat);
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn head_is(&self, con: &Constructor) -> bool {
        if self.0.is_empty() {
            false
        } else {
            match &self.0[0] {
                Pattern::Con(ref c, _) => c == con,
                _ => false,
            }
        }
    }

    pub fn head_is_con(&self) -> bool {
        if self.0.is_empty() {
            false
        } else {
            match &self.0[0] {
                Pattern::Con(_, _) => true,
                _ => false,
            }
        }
    }

    pub fn head_is_wcard(&self) -> bool {
        if self.0.is_empty() {
            false
        } else {
            match &self.0[0] {
                Pattern::WildCard => true,
                _ => false,
            }
        }
    }

    pub fn head_is_or(&self) -> bool {
        if self.0.is_empty() {
            false
        } else {
            match &self.0[0] {
                Pattern::Or(_, _) => true,
                _ => false,
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct PatternMatrix<'a> {
    columns: Vec<Row>,
    phantom:PhantomData<&'a ()>
}


impl <'a> PatternMatrix<'a> {
    pub fn new() -> Self {
        Self { columns: vec![],phantom:PhantomData }
    }

    pub fn add_row(&mut self, row: Row) {
        self.columns.push(row);
    }

    pub fn concat(&mut self, matrix: PatternMatrix) {
        self.columns.extend(matrix.columns)
    }

    /// Specialization by constructor `c` simplifies matrix `P` under the assumption  that `v1` admits `c` as  a  head  constructor
    pub fn specialization(&mut self, con: &Constructor) -> PatternMatrix {


        let mut matrice = PatternMatrix::new();

//        std::mem::swap(&mut old_rows, &mut self.columns); // swap the two so we can destructure the old row and build a new one

        for row in self.columns.iter() {
            let mut new_row = Row::new(Vec::new());

            if row.head_is(&con) {
                // check if the head is a constructor
                // if so remove the pattern from the matrix
                let head = &row.0[0];

                // add into the matrix the subterms of the constructor

                match head {
                    Pattern::Con(_, args) => {
                        args.iter().cloned().for_each(|arg| new_row.add(arg));
                    }
                    _ => unreachable!(),
                }

                for pattern in row.0.iter().cloned().skip(1) {
                    new_row.add(pattern);
                } // add the other patterns from the row that where present before
            } else if row.head_is_wcard() {
                for _ in 0..row.size() {
                    new_row.add(Pattern::WildCard);
                } // for the number of columns with this row add a wild card patter

                for pattern in row.0.iter().cloned().skip(1) {
                    // skip the head
                    new_row.add(pattern)
                } // add the other patterns from the row that where present before
            } else if row.head_is_or() {
                let head = &row.0[0]; // remove the `or` pattern from the matrix
                                      //TODO remove code duplication
                match head {
                    Pattern::Or(lhs, rhs) => {
                        // first create a matrix which the lhs pattern along with other patterns from this row
                        let mut lhs_matrix = PatternMatrix::new();

                        let mut new_row = Row::new(vec![*lhs.clone()]);

                        for pat in row.0.iter() {
                            new_row.add(pat.clone());
                        }

                        lhs_matrix.add_row(new_row);

                        lhs_matrix.specialization(con); // apply specialization to the matrix

                        let mut rhs_matrix = PatternMatrix::new();

                        let mut new_row = Row::new(vec![*rhs.clone()]);

                        for pat in row.0.iter().cloned() {
                            new_row.add(pat);
                        }

                        rhs_matrix.add_row(new_row);
                        //apply the lhs aswell

                        // add the two matrices to the current one
                        matrice.concat(lhs_matrix.specialization(con));
                        matrice.concat(rhs_matrix.specialization(con));
                    }

                    _ => unreachable!(),
                }
            }

            if new_row.is_empty() {
                continue;
            } else {
                matrice.add_row(new_row);
            }
        }

        matrice
    }

    /// The default matrix retains the rows of `P` whose first pattern `p^j1` admits all
    /// values `c′(v1, . . . , va)` as instances,where constructor `c′` is not present in
    /// the first column of `P`
    pub fn default(&mut self)  -> PatternMatrix {
        let mut matrice = PatternMatrix::new();

        for mut row in self.columns.iter() {
            let mut new_row = Row::new(vec![]);

            if row.head_is_con() {
                continue;
            } else if row.head_is_wcard() {
                for pattern in row.0.iter().cloned().skip(1) {
                    // skip the head
                    new_row.add(pattern)
                }
            } else {
                let head = &row.0[0];

                match head {
                    Pattern::Or(lhs, rhs) => {
                        // first create a matrix which the lhs pattern along with other patterns from this row
                        let mut lhs_matrix = PatternMatrix::new();

                        let mut new_row = Row::new(vec![*lhs.clone()]);

                        for pat in row.0.iter() {
                            new_row.add(pat.clone());
                        }

                        lhs_matrix.add_row(new_row);

                        lhs_matrix.default(); // apply specialization to the matrix

                        let mut rhs_matrix = PatternMatrix::new();

                        let mut new_row = Row::new(vec![*rhs.clone()]);

                        for pat in row.0.iter().cloned() {
                            new_row.add(pat);
                        }

                        rhs_matrix.add_row(new_row);
                        rhs_matrix.default(); //apply to the lhs as well

                        // add the two matrices to the current one
                        matrice.concat(lhs_matrix);
                        matrice.concat(rhs_matrix);
                    }

                    _ => unreachable!(),
                }
            }

            matrice.add_row(new_row);
        }

        matrice
    }
}

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

impl <'a> Display for PatternMatrix<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for col in self.columns.iter() {
            write!(f, "(")?;
            for pat in col.0.iter() {
                write!(f, "  ")?;
                write!(f, "{}", pat)?;
                write!(f, "  ")?;
            }
            write!(f, ")")?;
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pattern::Con(ref con, ref args) => {
                write!(f, "{}", con.name)?;

                if args.is_empty() {
                    return Ok(());
                }

                write!(f, "(")?;

                for (i, arg) in args.iter().enumerate() {
                    if i + 1 == args.len() {
                        write!(f, "{}", arg)?;
                    } else {
                        write!(f, "{},", arg)?;
                    }
                }

                write!(f, ")")
            }

            //            Pattern::Var(ref var) => write!(f, "{}", var),
            Pattern::WildCard => write!(f, "_"),
            Pattern::Or(ref lhs, ref rhs) => write!(f, "{} | {}", lhs, rhs),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
