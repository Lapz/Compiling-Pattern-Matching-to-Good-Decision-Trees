use std::fmt::{self, Display};
use std::ptr::hash;

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
    Var(String),                    // e.g foo /vars
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

#[derive(Debug)]
struct Row(Vec<Pattern>);

impl Row {
    pub fn new(pats: Vec<Pattern>) -> Self {
        Self(pats)
    }

    pub fn add(&mut self, pat: Pattern) {
        self.0.push(pat);
    }

    pub fn size(&self)  -> usize {
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
                Pattern::Or(_,_) => true,
                _ => false,
            }
        }
    }
}

#[derive(Debug)]
struct PatternMatrix {
    columns: Vec<Row>,
}

impl PatternMatrix {
    pub fn new() -> Self {
        Self { columns: vec![] }
    }

    pub fn add_row(&mut self, row: Row) {
        self.columns.push(row);
    }

    pub fn specialization(&mut self, con: Constructor) {
        let mut old_rows = Vec::new();

        std::mem::swap(&mut old_rows,&mut self.columns); // swap the two so we can destructure the old row and build a new one

        for mut row in old_rows.into_iter() { // for
            let mut new_row = Row::new(vec![]);

            if row.head_is(&con) {
                let head = row.0.remove(0);
                match head {
                    Pattern::Con(_, args) => {
                        args.into_iter().for_each(|arg| new_row.add(arg));
                    }
                    _ => unreachable!(),
                }

                for pattern in row.0.into_iter() {
                    new_row.add(pattern);
                }
            }else if row.head_is_wcard() {

                for _ in 0..row.size() {
                    new_row.add(Pattern::WildCard);
                }

                for pattern in row.0.into_iter().skip(1) { // skip the head
                    new_row.add(pattern)
                }
            }else if row.head_is_or() {
                let head = row.0.remove(0);

                match head {
                    Pattern::Or(lhs,rhs) => {

                    }
                }

            }

            if new_row.is_empty() {
                continue;
            }else {
                self.columns.push(new_row);
            }
        }

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

    matrix.add_row(Row::new(vec![list(vec![]), wcard()]));
    matrix.add_row(Row::new(vec![wcard(), list(vec![])]));
    matrix.add_row(Row::new(vec![split(wcard(),wcard()), split(wcard(),wcard())]));

    println!("{}", matrix);

   matrix.specialization(Constructor {
        name: "Split".into(),
        arity: 2,
        span: 1,
    });

    println!("{}",matrix);

    println!("Hello, world!");
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
                    return Ok(())
                }

                write!(f,"(")?;


                for (i, arg) in args.iter().enumerate() {
                    if i + 1 == args.len() {
                        write!(f, "{}", arg)?;
                    } else {
                        write!(f, "{},", arg)?;
                    }
                }

                write!(f, ")")
            }

            Pattern::Var(ref var) => write!(f, "{}", var),
            Pattern::WildCard => write!(f, "_"),
            Pattern::Or(ref lhs, ref rhs) => write!(f, "{} | {}", lhs, rhs),
        }
    }
}
