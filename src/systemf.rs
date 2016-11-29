use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;

type Id = String;

#[derive(Debug)]
pub enum EvaluationError {
    UnboundVariable(Rc<Id>),
    IllformedExpression,
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::EvaluationError::*;
        match *self {
            UnboundVariable(ref id) => write!(f, "Encountered unbound variable: {}", id),
            IllformedExpression => write!(f, "Illformed expression"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Var(Rc<Id>),
    Fun(Rc<Type>, Rc<Type>),
    Forall(Rc<Id>, Rc<Type>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Var(Rc<Id>),
    Lam(Rc<Id>, Rc<Type>, Rc<Expr>),
    App(Rc<Expr>, Rc<Expr>),
    TLam(Rc<Id>, Rc<Expr>),
    TApp(Rc<Expr>, Rc<Type>),
    Let(Rc<Id>, Rc<Type>, Rc<Expr>, Rc<Expr>),
}

impl Expr {
    fn eval(e: Rc<Expr>,
            emap: &mut HashMap<&str, Rc<Expr>>,
            tmap: &mut HashMap<&str, Rc<Type>>)
            -> Result<Rc<Expr>, EvaluationError> {
        use self::EvaluationError::*;

        match *e {
            Expr::Var(ref id) => {
                match emap.get::<str>(id) {
                    Some(ref e) => Ok((*e).clone()),
                    None => Err(UnboundVariable(id.clone())), 
                }
            }
            Expr::Lam(_, _, _) |
            Expr::TLam(_, _) => Ok(e.clone()),
            Expr::App(ref e1, ref e2) => {
                let v1 = Expr::eval(e1.clone(), emap, tmap)?;
                let v2 = Expr::eval(e2.clone(), emap, tmap)?;
                match *v1 {
                    Expr::Lam(ref x, _, ref e3) => {
                        let mut new_map = emap.clone();
                        new_map.insert(x, e3.clone());
                        Expr::eval(e2.clone(), &mut new_map, tmap)
                    },
                    _ => Err(IllformedExpression),
                }
            }
            Expr::TApp(ref e, ref t) => {
                let v = Expr::eval(e.clone(), emap, tmap)?;
                match *v {
                    Expr::TLam(ref x, ref e) => {
                        let mut new_map = tmap.clone();
                        new_map.insert(x, t.clone());
                        Expr::eval(e.clone(), emap, &mut new_map)
                    },
                    _ => Err(IllformedExpression),
                } 
            },
            Expr::Let(ref x, ref t, ref e1, ref e2) => {
                let desugared = Expr::App(Rc::new(Expr::Lam(x.clone(), t.clone(), e2.clone())),
                                          e1.clone());
                Expr::eval(Rc::new(desugared), emap, tmap)
            },
        }
    }
}