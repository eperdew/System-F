use std::collections::HashMap;
use std::collections::HashSet;
use std::borrow::BorrowMut;
use std::fmt;
use std::fmt::Display;
use std::cmp;

type Id = String;

/// Returns an `Id` with prefix `x` that is not in the set `vars`
fn fresh_var_like(x: &Id, vars: HashSet<Id>) -> Id {
    let mut suffix = 0;
    let mut result = x.clone();
    let len = result.len();
    loop {
        result.push_str(&suffix.to_string());

        if !vars.contains(&result) {
            return result;
        }

        suffix += 1;
        result.truncate(len);
    }
}

#[derive(Debug)]
pub enum EvaluationError {
    UnboundVariable(String),
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
    Var(Id),
    Fun(Box<Type>, Box<Type>),
    Forall(Id, Box<Type>),
}

impl PartialEq for Type {
    /// Equality is defined as alpha-equivalence of types
    fn eq(&self, other: &Type) -> bool {
        use self::Type::*;

        match (self,other) {
            (&Var(ref x),&Var(ref y)) => {
                *x == *y
            },
            (&Fun(ref t1, ref t2),&Fun(ref t3, ref t4)) => {
                *t1 == *t3 && *t2 == *t4 
            },
            (&Forall(ref A, ref t1),&Forall(ref B, ref t2)) => {
                t1.subst(&Var(B.clone()), &A) == **t2
            },
            _ => false,
        }
    }
}

impl Eq for Type { }

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Var(ref x) => write!(f,"{}",x),
            Type::Fun(ref a, ref b) => write!(f,"({}) -> {}",a,b),
            Type::Forall(ref x, ref t) => write!(f,"(forall {}. {})", x, t),
        }
    }
}

impl Type {
    /// Returns the result of the capture avoiding substitution `self` {`t`/`X`}
    fn subst(&self, t: &Type, X: &str) -> Type {
        match *self {
            Type::Var(ref Y) => {
                if X == *Y {
                    t.clone()
                } else {
                    self.clone()
                }
            },
            Type::Fun(ref t1, ref t2) => {
                Type::Fun(Box::new(t1.subst(t,X)), Box::new(t2.subst(t,X)))
            },
            Type::Forall(ref Y, ref tau) => {
                if X == *Y {
                    self.clone()
                } else {
                    let mut fvs = t.free_vars();
                    if fvs.contains(Y) {
                        let mut vars = self.vars();
                        vars.extend(fvs.drain());
                        vars.insert(String::from(X));
                        let fresh_id = fresh_var_like(Y, vars);
                        let renamed = self.rename(Y, &fresh_id);
                        renamed.subst(t, X)
                    } else {
                        Type::Forall(Y.clone(), Box::new(tau.subst(t,X)))
                    }
                }
            },
        }
    }

    /// Returns an identical type with `var` renamed into `into` 
    fn rename(&self, var: &str, into: &str) -> Type {
        match *self {
            Type::Var(ref X) => {
                if *X == var {
                    Type::Var(String::from(into))
                } else {
                    self.clone()
                }
            },
            Type::Fun(ref t1, ref t2) => {
                Type::Fun(Box::new(t1.rename(var, into)),
                        Box::new(t2.rename(var,into)))
            },
            Type::Forall(ref X, ref tau) => {
                let binder = {
                    if *X == var {
                        String::from(into)       
                    } else {
                        X.clone()
                    }
                };
                Type::Forall(binder, Box::new(tau.rename(var, into)))
            },
        }
    }

    /// Returns a set the free variables contained in `self`
    fn free_vars(&self) -> HashSet<Id> {
        match *self {
            Type::Var(ref X) => {
                let mut fvs = HashSet::new();
                fvs.insert(X.clone());
                fvs
            },
            Type::Fun(ref t1, ref t2) => {
                let mut fvs = t1.free_vars();
                let mut t2_fvs = t2.free_vars();
                fvs.extend(t2_fvs.drain());
                fvs
            }
            Type::Forall(ref X, ref t) => {
                let mut fvs = t.free_vars();
                fvs.remove(X);
                fvs
            }
        }
    }

    /// Returns a set containing all of the variables contained in `self`
    fn vars(&self) -> HashSet<Id> {
        match *self {
            Type::Var(ref X) => {
                let mut vars = HashSet::new();
                vars.insert(X.clone());
                vars
            },
            Type::Fun(ref t1, ref t2) => {
                let mut vars = t1.vars();
                let mut t2_vars = t2.vars();
                vars.extend(t2_vars.drain());
                vars
            }
            Type::Forall(ref X, ref t) => {
                let mut vars = t.vars();
                vars.insert(X.clone());
                vars
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Var(Id),
    Lam(Id, Box<Type>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    TLam(Id, Box<Expr>),
    TApp(Box<Expr>, Box<Type>),
    Let(Id, Box<Type>, Box<Expr>, Box<Expr>),
    TLet(Id, Box<Type>, Box<Expr>),
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
    /// Returns the normal form of `self` under call-by-value semantics
    pub fn eval(&self) -> Result<Expr,EvaluationError> {
        match *self {
            Expr::Var(ref x) => {
                Err(EvaluationError::UnboundVariable(x.clone()))
            },
            Expr::Lam(_,_,_) | Expr::TLam(_,_) => Ok(self.clone()),
            Expr::App(ref e1, ref e2) => {
                let v1 = e1.eval()?;
                let v2 = e2.eval()?;
                if let Expr::Lam(x,t,e3) = v1 {
                    let new_expr = e3.subst(&v2, &x);
                    new_expr.eval()
                } else {
                    Err(EvaluationError::IllformedExpression)
                }
            },
            Expr::TApp(ref e, ref t) => {
                let v = e.eval()?;
                if let Expr::TLam(X,e) = v {
                    let new_expr = e.subst_type(t, &X);
                    new_expr.eval()
                } else {
                    Err(EvaluationError::IllformedExpression)
                }
            },
            Expr::Let(ref x, ref t, ref e1, ref e2) => {
                Expr::App(
                    Box::new(Expr::Lam(
                        x.clone(),
                        t.clone(),
                        e2.clone())),
                    e1.clone()).eval()
            },
            Expr::TLet(ref X, ref t, ref e) => {
                Expr::TApp(
                    Box::new(Expr::TLam(
                        X.clone(),
                        e.clone())),
                    t.clone()).eval()
            },
        }
    }

    /// Returns the result of the capture avoiding substitution `self` {`e`/`x`}
    fn subst(&self, e: &Expr, x: &str) -> Expr {
        match *self {
            Expr::Var(ref y) => {
                if x == *y {
                    e.clone()
                } else {
                    self.clone()
                }
            },
            Expr::Lam(ref y,ref t,ref e1) => {
                if x == *y {
                    self.clone()
                } else {
                    // TODO: check freshness
                    unimplemented!()
                }
            },
            Expr::App(ref e1,ref e2) => {
                Expr::App(Box::new(e1.subst(e,x)), Box::new(e2.subst(e,x)))   
            },
            Expr::TLam(ref X,ref e1) => {
                Expr::TLam(X.clone(), Box::new(e1.subst(e,x)))
            },
            Expr::TApp(ref e1,ref t) => {
                Expr::TApp(Box::new(e1.subst(e,x)), t.clone())
            },
            Expr::Let(ref y,ref t,ref e1,ref e2) => {
                // Unclear
                unimplemented!()
            },
            Expr::TLet(ref X,ref t,ref e1) => {
                Expr::TLet(X.clone(), t.clone(), Box::new(e1.subst(e,x)))
            },
        }
    }

    fn subst_type(&self, to: &Type, from: &str) -> Expr {
        match *self {
            Expr::Var(ref y) => {
                self.clone()
            },
            Expr::Lam(ref y,ref t,ref e1) => {
                let new_t = t.subst(to, from);
                Expr::Lam(y.clone(), Box::new(new_t), Box::new(e1.subst_type(to,from)))
            },
            Expr::App(ref e1,ref e2) => {
                Expr::App(Box::new(e1.subst_type(to,from)), Box::new(e2.subst_type(to,from)))   
            },
            Expr::TLam(ref X,ref e1) => {
                if *X == from {
                    self.clone()
                } else {
                    // FIXME: Do I need to worry about the fvs of `to` here?
                    Expr::TLam(X.clone(), Box::new(e1.subst_type(to,from)))
                }
            },
            Expr::TApp(ref e1,ref t) => {
                let new_t = t.subst(to, from);
                let new_e1 = e1.subst_type(to, from);
                Expr::TApp(Box::new(new_e1), Box::new(new_t))
            },
            Expr::Let(ref y,ref t,ref e1,ref e2) => {
                unimplemented!()
            },
            Expr::TLet(ref X,ref t,ref e1) => {
                unimplemented!()
            },
        }
    }
}