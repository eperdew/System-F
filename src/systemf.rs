use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::borrow::BorrowMut;
use std::fmt;
use std::fmt::Display;

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

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Var(ref x) => write!(f,"{}",x),
            Type::Fun(ref a, ref b) => write!(f,"({}) -> {}",a,b),
            Type::Forall(ref x, ref t) => write!(f,"(forall {}. {})", x, t),
        }
    }
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

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Var(ref x) => write!(f,"{}",x),
            Expr::Lam(ref x, ref t, ref e) => write!(f,"(\\{}:{}.{})",x,t,e),
            Expr::App(ref e1, ref e2) => write!(f,"({} {})",e1,e2),
            Expr::TLam(ref X, ref e) => write!(f,"(/\\{}.{})",X,e),
            Expr::TApp(ref e, ref t) => write!(f,"({} [{}])",e,t),
            Expr::Let(ref x, ref t, ref e1, ref e2) => write!(f,"(let {}:{} = {} in {})",x,t,e1,e2),
            Expr::TLet(ref X, ref t, ref e) => write!(f,"(Let {} = {} in {})",X,t,e),
        }
    }
}

impl Expr {
    /// Evaluates an expression and returns either the normal form or an error
    /// if it is illformed
    pub fn eval(self) -> Result<Rc<Expr>, EvaluationError> {
        Expr::eval_expr(Rc::new(self))
    }

    /// Evaluates an expression and returns either the normal form or an error 
    /// if it is illformed
    pub fn eval_expr(e: Rc<Expr>) -> Result<Rc<Expr>, EvaluationError> {
        Expr::eval_helper(e, &mut HashMap::new(), &mut HashMap::new())
    }

    /// Workhorse for `eval` and `eval_expr` 
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

    /// Recursively expand the free variables of the types of the expression 
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

    pub fn type_check(&self) -> Option<Type> {
        self.type_check_helper(&HashMap::new(), &HashMap::new(), &HashSet::new())
    }

    fn type_check_helper(&self, 
        emap: &HashMap<&str,&Type>, 
        tmap: &HashMap<&str,&Type>,
        bvs: &HashSet<&str>) -> Option<Type> {
        match *self {
            Expr::Var(ref id) => {
                emap.get::<str>(&**id).map(|t| { (*t).clone() } )
            },
            Expr::Lam(ref x, ref t, ref e) => {
                let mut new_emap = emap.clone();
                new_emap.insert(x, t);
                let tbod = e.type_check_helper(&new_emap, tmap, bvs);
                tbod.map(|tb| { Type::Fun((*t).clone(), Rc::new(tb)) })
            },
            Expr::App(ref e1, ref e2) => {
                let t1 = e1.type_check_helper(emap, tmap, bvs);
                let t2 = e2.type_check_helper(emap, tmap, bvs);
                match t1 {
                    Some(Type::Fun(ta,tb)) => match t2 {
                        Some(tc) => {
                            if Type::alpha_equiv(&ta,&tc) {
                                Some((*tb).clone())
                            }
                            else {
                                None
                            }
                        },
                        _ => None,
                    },
                    _ => None,
                }
            },
            Expr::TLam(ref X, ref e) => {
                let mut new_bvs = bvs.clone();
                new_bvs.insert(X);
                e.type_check_helper(emap, tmap, &new_bvs)
                    .map(|t| { Type::Forall((*X).clone(),Rc::new(t)) })
            },
            Expr::TApp(ref e, ref t) => {
                let t1 = e.type_check_helper(emap, tmap, bvs);
                match t1 {
                    Some(Type::Forall(X,t2)) => {
                        let mut new_tmap = tmap.clone();
                        new_tmap.insert(&*X, t);
                        let new_tmap = {
                            let mut map: HashMap<&str,Rc<Type>> = HashMap::new();
                            for (k,v) in new_tmap {
                                map.insert(k, Rc::new((*v).clone()));
                            }
                            map
                        };
                        Some((*Type::eval(t2, &new_tmap, bvs).unwrap()).clone())
                    },
                    _ => None,
                }
            },
            Expr::Let(ref x, ref t, ref e1, ref e2) => {
                (Expr::App(
                    Rc::new(Expr::Lam(x.clone(),t.clone(),e2.clone())),
                    e1.clone())).type_check_helper(emap, tmap, bvs)
            },
            Expr::TLet(ref X, ref t, ref e) => {
                (Expr::TApp(
                    Rc::new(Expr::TLam(X.clone(),e.clone())),
                    t.clone())).type_check_helper(emap, tmap, bvs)
            },
        }
    }
}

impl Type {
    /// Recursively expand the free variables of a type using the given map
    fn eval(t: Rc<Type>, map: &HashMap<&str,Rc<Type>>, bvs: &HashSet<&str>)
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

    fn alpha_equiv(t1: &Type, t2: &Type) -> bool {
        // TODO
        unimplemented!()
    }
}