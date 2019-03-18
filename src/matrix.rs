use crate::pattern::{Constructor, Pattern};
use std::collections::HashSet;
use std::fmt::{self, Display};

pub type Expr = usize;
#[derive(Debug, PartialEq)]
pub struct Row(Vec<Pattern>, Expr);

#[derive(Debug, PartialEq)]
pub struct PatternMatrix {
    columns: Vec<Row>,
}

impl Row {
    pub fn new(pats: Vec<Pattern>, returns: usize) -> Self {
        Self(pats, returns)
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

    pub fn action(&self) -> Expr {
        self.1
    }

    pub fn contains_wcard_only(&self) -> bool {
        for pat in self.0.iter() {
            match pat {
                Pattern::WildCard => continue,
                _ => {
                    return false;
                }
            }
        }

        true
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

impl PatternMatrix {
    pub fn new() -> Self {
        Self { columns: vec![] }
    }

    pub fn add_row(&mut self, row: Row) {
        self.columns.push(row);
    }

    pub fn get(&self, index: usize) -> &Row {
        &self.columns[index]
    }

    pub fn concat(&mut self, matrix: PatternMatrix) {
        self.columns.extend(matrix.columns)
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    pub fn swap(&mut self, index: usize) {
        for i in 0..self.columns.len() {

                       unsafe {
                           // swap
                           // Can't take two mutable loans from one vector, so instead just cast
                           // them to their raw pointers to do the swap
                           let x:*mut Pattern = &mut self.columns[i].0[index];
                           let y:*mut Pattern = &mut self.columns[i].0[1];

                           std::ptr::swap(x,y);
                       }
        }
    }

    /// All columns that have no wildcard pattern;
    pub fn cols_with_wcard(&self) -> Vec<usize> {

        let mut coords = Vec::new();

        for (i, row) in self.columns.iter().rev().enumerate() {


            let mut not_wcard = false;


            for (j,pat) in row.0.iter().enumerate() {
                match pat {
                    Pattern::WildCard => (),
                    _ => coords.push(j*i)
                }
            }
        }


        let mut coords = coords.into_iter().collect::<HashSet<_>>();

        let mut coords = coords.into_iter().collect::<Vec<_>>();

        coords.sort();

        coords
    }

    pub fn head_cons(&self, index: usize) -> HashSet<Constructor> {
        let mut set = HashSet::new();

        for (i, col) in self.columns.iter().enumerate() {
            set.extend(self.columns[i].0[index].con());
        }

        set
    }

    /// Specialization by constructor `c` simplifies matrix `P` under the assumption  that `v1` admits `c` as  a  head  constructor
    pub fn specialization(&mut self, con: &Constructor) -> PatternMatrix {
        let mut matrice = PatternMatrix::new();

        //        std::mem::swap(&mut old_rows, &mut self.columns); // swap the two so we can destructure the old row and build a new one

        for row in self.columns.iter() {
            let mut new_row = Row::new(Vec::new(), row.1);

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

                        let mut new_row = Row::new(vec![*lhs.clone()], row.1);

                        for pat in row.0.iter() {
                            new_row.add(pat.clone());
                        }

                        lhs_matrix.add_row(new_row);

                        lhs_matrix.specialization(con); // apply specialization to the matrix

                        let mut rhs_matrix = PatternMatrix::new();

                        let mut new_row = Row::new(vec![*rhs.clone()], row.1);

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
    pub fn default(&mut self) -> PatternMatrix {
        let mut matrice = PatternMatrix::new();

        for row in self.columns.iter() {
            let mut new_row = Row::new(vec![], row.1);

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

                        let mut new_row = Row::new(vec![*lhs.clone()], row.1);

                        for pat in row.0.iter() {
                            new_row.add(pat.clone());
                        }

                        lhs_matrix.add_row(new_row);

                        lhs_matrix.default(); // apply specialization to the matrix

                        let mut rhs_matrix = PatternMatrix::new();

                        let mut new_row = Row::new(vec![*rhs.clone()], row.1);

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

impl Display for PatternMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for col in self.columns.iter() {
            write!(f, "(")?;
            for pat in col.0.iter() {
                write!(f, "  ")?;
                write!(f, "{}", pat)?;
                write!(f, "  ")?;
            }
            write!(f, "-> {}", col.1)?;
            write!(f, ")")?;
            write!(f, "\n")?;
        }

        Ok(())
    }
}
