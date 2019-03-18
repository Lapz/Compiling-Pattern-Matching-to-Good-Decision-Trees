use std::collections::HashSet;
use std::fmt::{self, Display};

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
pub struct Constructor {
    pub name: String,
    pub arity: i32,
    pub span: i32,
}

/// A pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    //    Var(String),                    // e.g foo /vars
    Con(Constructor, Vec<Pattern>), //e.g. List(10,Nil)
    WildCard,
    Or(Box<Pattern>, Box<Pattern>),
}

pub fn nil() -> Pattern {
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
pub fn split(lhs: Pattern, rhs: Pattern) -> Pattern {
    let con = Constructor {
        name: "Split".into(),
        arity: 2,
        span: 1,
    };

    Pattern::Con(con, vec![lhs, rhs])
}

pub fn list(args: Vec<Pattern>) -> Pattern {
    let con = Constructor {
        name: "List".into(),
        arity: args.len() as i32,
        span: 1,
    };

    Pattern::Con(con, args)
}

pub fn wcard() -> Pattern {
    Pattern::WildCard
}

impl Pattern {
    pub fn con(&self) -> HashSet<Constructor> {
        match self {
            Pattern::WildCard => HashSet::new(),
            Pattern::Con(ref con, _) => {
                let mut set = HashSet::new();
                set.insert(con.clone());
                set
            }
            Pattern::Or(ref lhs, ref rhs) => {
                let set = lhs.con();
                set.union(&rhs.con());

                set
            }
        }
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
