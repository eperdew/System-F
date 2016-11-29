use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Var(Rc<Id>),
    Fun(Rc<Type>, Rc<Type>),
    Forall(Rc<Id>, Rc<Type>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Var(Rc<Id>),
    Lam(Rc<Id>, Rc<Type>, Rc<Expr>),
    App(Rc<Expr>, Rc<Expr>),
    TLam(Rc<Id>, Rc<Expr>),
    TApp(Rc<Expr>, Rc<Type>),
    Let(Rc<Id>, Rc<Type>, Rc<Expr>, Rc<Expr>),
    TLet(Rc<Id>, Rc<Type>, Rc<Expr>),
}

impl Expr {
    pub fn eval(self) -> Result<Rc<Expr>, EvaluationError> {
        Expr::eval_expr(Rc::new(self))
    }

    pub fn eval_expr(e: Rc<Expr>) -> Result<Rc<Expr>, EvaluationError> {
        Expr::eval_helper(e, &mut HashMap::new(), &mut HashMap::new())
    }

    fn eval_helper(e: Rc<Expr>,
            emap: &mut HashMap<&str, Rc<Expr>>,
            tmap: &mut HashMap<&str, Rc<Type>>)
            -> Result<Rc<Expr>, EvaluationError> {
        use self::EvaluationError::*;

        let result = match *e {
            Expr::Var(ref id) => {
                match emap.get::<str>(id) {
                    Some(ref e) => Ok((*e).clone()),
                    None => Err(UnboundVariable(id.clone())), 
                }
            }
            Expr::Lam(_, _, _) |
            Expr::TLam(_, _) => Ok(e.clone()),
            Expr::App(ref e1, ref e2) => {
                let v1 = Expr::eval_helper(e1.clone(), emap, tmap)?;
                let v2 = Expr::eval_helper(e2.clone(), emap, tmap)?;
                match *v1 {
                    Expr::Lam(ref x, _, ref e3) => {
                        let mut new_map = emap.clone();
                        new_map.insert(x, e3.clone());
                        Expr::eval_helper(e2.clone(), &mut new_map, tmap)
                    },
                    _ => Err(IllformedExpression),
                }
            }
            Expr::TApp(ref e, ref t) => {
                let v = Expr::eval_helper(e.clone(), emap, tmap)?;
                match *v {
                    Expr::TLam(ref x, ref e) => {
                        let mut new_map = tmap.clone();
                        new_map.insert(x, t.clone());
                        Expr::eval_helper(e.clone(), emap, &mut new_map)
                    },
                    _ => Err(IllformedExpression),
                } 
            },
            Expr::Let(ref x, ref t, ref e1, ref e2) => {
                let desugared = Expr::App(Rc::new(Expr::Lam(x.clone(), t.clone(), e2.clone())),
                                          e1.clone());
                Expr::eval_helper(Rc::new(desugared), emap, tmap)
            },
            Expr::TLet(ref X, ref t, ref e) => {
                let desugared = Expr::TApp(Rc::new(Expr::TLam(X.clone(), e.clone())), t.clone());
                Expr::eval_helper(Rc::new(desugared), emap, tmap)
            }
        }?;

        Expr::expand_types(result, tmap, &mut HashSet::new())
    }

    fn expand_types(e: Rc<Expr>, map: &HashMap<&str,Rc<Type>>, bvs: &mut HashSet<&str>) 
    -> Result<Rc<Expr>,EvaluationError> {
        match *e {
            Expr::Var(_) => {
                Ok(e.clone())
            },
            Expr::App(ref e1, ref e2) => {
                let r1 = Expr::expand_types(e1.clone(), map, bvs)?;
                let r2 = Expr::expand_types(e2.clone(), map, bvs)?;
                Ok(Rc::new(Expr::App(r1,r2)))
            },
            Expr::TApp(ref e, ref t) => {
                let re = Expr::expand_types(e.clone(), map, bvs)?;
                let te = Type::eval(t.clone(), map, bvs)?;
                Ok(Rc::new(Expr::TApp(re, te)))
            },
            Expr::Let(ref x, ref t, ref e1, ref e2) => {
                let r1 = Expr::expand_types(e1.clone(), map, bvs)?;
                let r2 = Expr::expand_types(e2.clone(), map, bvs)?;
                let te = Type::eval(t.clone(), map, bvs)?;
                Ok(Rc::new(Expr::Let(x.clone(), te, r1, r2)))
            },
            Expr::TLet(ref X, ref t, ref e) => {
                let te = Type::eval(t.clone(), map, bvs)?;
                let mut new_bvs = bvs.clone();
                new_bvs.insert(X);
                let re = Expr::expand_types(e.clone(), map, &mut new_bvs)?;
                Ok(Rc::new(Expr::TLet(X.clone(), te, re)))
            },  
            Expr::Lam(ref x, ref t, ref e) => {
                let te = Type::eval(t.clone(), map, bvs)?;
                let re = Expr::expand_types(e.clone(), map, bvs)?;
                Ok(Rc::new(Expr::Lam(x.clone(), te, re)))
            },
            Expr::TLam(ref X, ref e) => {
                let mut new_bvs = bvs.clone();
                new_bvs.insert(X);
                let re = Expr::expand_types(e.clone(), map, &mut new_bvs)?;
                Ok(Rc::new(Expr::TLam(X.clone(), re)))
            },
        }
    }
}

impl Type {
    fn eval(t: Rc<Type>, map: &HashMap<&str,Rc<Type>>, bvs: &mut HashSet<&str>)
    -> Result<Rc<Type>,EvaluationError> {
        match *t {
            Type::Var(ref id) => {
                if bvs.contains::<str>(id) {
                    Ok(t.clone())
                } else {
                    match map.get::<str>(id) {
                        Some(ref t) => Type::eval((*t).clone(), map, bvs),
                        None => Err(EvaluationError::UnboundVariable(id.clone())),
                    }
                }
            },
            Type::Fun(ref t1, ref t2) => {
                let rt1 = Type::eval(t1.clone(), map, bvs)?;
                let rt2 = Type::eval(t2.clone(), map, bvs)?;
                Ok(Rc::new(Type::Fun(rt1, rt2)))
            },
            Type::Forall(ref id, ref t) => {
                let mut new_bvs = bvs.clone();
                new_bvs.insert(&**id);
                let rt = Type::eval(t.clone(), map, &mut new_bvs)?;
                Ok(Rc::new(Type::Forall(id.clone(), rt)))
            },
        }
    }
}