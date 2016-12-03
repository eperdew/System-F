#![allow(non_snake_case)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

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
pub enum TypeError {
    UnboundVariable(String),
    UnboundTypeVariable(String),
    IllegalApplication,
    IllegalTypeApplication,
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
                        fvs.insert(String::from(X));
                        let fresh_id = fresh_var_like(Y, fvs);
                        let fresh_var = Type::Var(fresh_id.clone());
                        let res = tau.subst(&fresh_var, Y).subst(t, X);
                        Type::Forall(fresh_id, Box::new(res))
                    } else {
                        Type::Forall(Y.clone(), Box::new(tau.subst(t,X)))
                    }
                }
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
    /// Returns the normal form of `self` using applicative order.
    ///
    /// Returns the expression unchanged if there are no defined semantics.
    pub fn eval(&self) -> Expr {
        match *self {
            Expr::Var(ref x) => {
                self.clone()
            },
            Expr::Lam(ref x, ref t, ref e) => {
                let v = e.eval();
                Expr::Lam(x.clone(), t.clone(), Box::new(v))
            },
            Expr::TLam(ref X, ref e) => {
                let v = e.eval();
                Expr::TLam(X.clone(), Box::new(v))
            },
            Expr::App(ref e1, ref e2) => {
                let v1 = e1.eval();
                let v2 = e2.eval();
                match v1 {
                    Expr::Lam(x,_,e3) => {
                        e3.subst(&v2, &x).eval()
                    },
                    _ => self.clone()
                }
            },
            Expr::TApp(ref e, ref t) => {
                let v = e.eval();
                match v {
                    Expr::TLam(X,e1) => {
                        let new_expr = e1.subst_type(t, &X);
                        new_expr.eval()
                    },
                    _ => self.clone()
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
                e.subst_type(t, X).eval()
            },
        }
    }

    /// Returns a `Type` if the given expression can be typed in System F with
    /// a closed term
    pub fn type_check(&self) -> Result<Type,TypeError> {
        let mut fvs = self.free_type_vars();
        for var in fvs.drain() {
            return Err(TypeError::UnboundTypeVariable(var));
        }
        self.type_check_helper(&HashMap::new())
    }

    fn type_check_helper(&self, tmap: &HashMap<&str,&Type>) -> Result<Type,TypeError> {
        match *self {
            Expr::Var(ref x) => {
                match tmap.get::<str>(x) {
                    Some(t) => { Ok((*t).clone()) },
                    None => Err(TypeError::UnboundVariable(x.clone())),
                }
            },
            Expr::Lam(ref x,ref t,ref e) => {
                let mut new_tmap = tmap.clone();
                new_tmap.insert(x,t);
                let t_e = e.type_check_helper(&new_tmap)?;
                Ok(Type::Fun(t.clone(),Box::new(t_e)))
            },
            Expr::App(ref e1,ref e2) => {
                let t_e1 = e1.type_check_helper(tmap)?;
                let t_e2 = e2.type_check_helper(tmap)?;
                match t_e1 {
                    Type::Fun(ta,tb) => {
                        if *ta == t_e2 {
                            Ok(*tb.clone())
                        } else {
                            Err(TypeError::IllegalApplication)
                        }
                    },
                    _ => Err(TypeError::IllegalApplication),
                }
            },
            Expr::TLam(ref X,ref e) => {
                let t_e = e.type_check_helper(tmap)?;
                Ok(Type::Forall(X.clone(), Box::new(t_e)))
            },
            Expr::TApp(ref e,ref t) => {
                let t_e = e.type_check_helper(tmap)?;
                match t_e {
                    Type::Forall(ref X, ref t1) => {
                        Ok(t1.subst(t, X))
                    },
                    _ => {
                        Err(TypeError::IllegalTypeApplication)
                    },
                }
            },
            Expr::Let(ref x,ref t,ref e1,ref e2) => {
                Expr::App(
                    Box::new(Expr::Lam(x.clone(), t.clone(), e2.clone())),
                    e1.clone()).type_check_helper(tmap)
            },
            Expr::TLet(ref X,ref t,ref e) => {
                e.subst_type(t, X).type_check_helper(tmap)
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
                    let mut fvs = e.free_vars();
                    if fvs.contains(y) {
                        fvs.insert(String::from(x));
                        let fresh_id = fresh_var_like(y, fvs);
                        let fresh_var = Expr::Var(fresh_id.clone());
                        let res = e1.subst(&fresh_var, y).subst(e, x);
                        Expr::Lam(fresh_id, t.clone(), Box::new(res))
                    } else {
                        Expr::Lam(y.clone(), t.clone(), Box::new(e1.subst(e, x)))
                    }
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
                let new_e1 = e1.subst(e, x);

                if x == *y {
                    Expr::Let(y.clone(), t.clone(), Box::new(new_e1), e2.clone())
                }
                else {
                    let mut fvs = e.free_vars();
                    if fvs.contains(y) {
                        fvs.insert(String::from(x));
                        let fresh_id = fresh_var_like(y, fvs);
                        let fresh_var = Expr::Var(fresh_id.clone());
                        let new_e2 = e2.subst(&fresh_var, y).subst(e, x);
                        Expr::Let(fresh_id, t.clone(), Box::new(new_e1), Box::new(new_e2))
                    }
                    else {
                        Expr::Let(y.clone(), t.clone(), Box::new(new_e1), Box::new(e2.subst(e, x)))
                    }
                }
            },
            Expr::TLet(ref X,ref t,ref e1) => {
                Expr::TLet(X.clone(), t.clone(), Box::new(e1.subst(e,x)))
            },
        }
    }

    fn subst_type(&self, to: &Type, from: &str) -> Expr {
        match *self {
            Expr::Var(_) => {
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
                    let mut fvs = to.free_vars();
                    if fvs.contains(X) {
                        fvs.insert(String::from(from));
                        let fresh_id = fresh_var_like(X, fvs);
                        let fresh_var = Type::Var(fresh_id.clone());
                        let new_e1 = e1.subst_type(&fresh_var, X).subst_type(to, from);
                        Expr::TLam(fresh_id, Box::new(new_e1))
                    } else {
                        Expr::TLam(X.clone(), Box::new(e1.subst_type(to, from)))
                    }
                }
            },
            Expr::TApp(ref e1,ref t) => {
                let new_t = t.subst(to, from);
                let new_e1 = e1.subst_type(to, from);
                Expr::TApp(Box::new(new_e1), Box::new(new_t))
            },
            Expr::Let(ref y,ref t,ref e1,ref e2) => {
                let new_t = t.subst(to, from);
                let new_e1 = e1.subst_type(to, from);
                let new_e2 = e2.subst_type(to, from);
                Expr::Let(y.clone(), Box::new(new_t), Box::new(new_e1), Box::new(new_e2))
            },
            Expr::TLet(ref X,ref t,ref e1) => {
                let new_t = t.subst(to, from);
                if *X == from {
                    Expr::TLet(X.clone(), Box::new(new_t), e1.clone())
                }
                else {
                    let mut fvs = to.free_vars();
                    if fvs.contains(X) {
                        fvs.insert(String::from(from));
                        let fresh_id = fresh_var_like(X, fvs);
                        let fresh_var = Type::Var(fresh_id.clone());
                        let new_e1 = e1.subst_type(&fresh_var, X).subst_type(to, from);
                        Expr::TLet(fresh_id, Box::new(new_t), Box::new(new_e1))
                    } else {
                        Expr::TLet(X.clone(), Box::new(new_t), Box::new(e1.subst_type(to, from)))
                    }
                }
            },
        }
    }

    /// Returns a set the free variables contained in `self`
    fn free_vars(&self) -> HashSet<Id> {
        match *self {
            Expr::Var(ref x) => {
                let mut vars = HashSet::new();
                vars.insert(x.clone());
                vars
            },
            Expr::Lam(ref x, _, ref e) => {
                let mut vars = e.free_vars();
                vars.remove(x);
                vars
            },
            Expr::App(ref e1, ref e2) => {
                let mut vars = e1.free_vars();
                let mut e2_vars = e2.free_vars();
                vars.extend(e2_vars.drain());
                vars
            },
            Expr::TLam(_, ref e) => {
                e.free_vars()
            },
            Expr::TApp(ref e, _) => {
                e.free_vars()
            },
            Expr::Let(ref x, _, ref e1, ref e2) => {
                let mut vars = e1.free_vars();
                let mut e2_vars = e2.free_vars();
                e2_vars.remove(x);
                vars.extend(e2_vars.drain());
                vars
            },
            Expr::TLet(_, _, ref e) => {
                e.free_vars()
            }
        }
    }

    /// Returns a set the free type variables contained in `self`
    fn free_type_vars(&self) -> HashSet<Id> {
        match *self {
            Expr::Var(_) => {
                HashSet::new()
            },
            Expr::Lam(_, ref t, ref e) => {
                let mut vars = t.free_vars();
                let mut e_vars = e.free_type_vars();
                vars.extend(e_vars.drain());
                vars
            },
            Expr::App(ref e1, ref e2) => {
                let mut vars = e1.free_type_vars();
                let mut e2_vars = e2.free_type_vars();
                vars.extend(e2_vars.drain());
                vars
            },
            Expr::TLam(ref X, ref e) => {
                let mut vars = e.free_type_vars();
                vars.remove(X);
                vars
            },
            Expr::TApp(ref e, ref t) => {
                let mut vars = e.free_type_vars();
                let mut t_vars = t.free_vars();
                vars.extend(t_vars.drain());
                vars
            },
            Expr::Let(_, ref t, ref e1, ref e2) => {
                let mut vars = t.free_vars();
                let mut e1_vars = e1.free_type_vars();
                let mut e2_vars = e2.free_type_vars();
                vars.extend(e1_vars.drain());
                vars.extend(e2_vars.drain());
                vars
            },
            Expr::TLet(ref X, ref t, ref e) => {
                let mut vars = t.free_vars();
                let mut e_vars = e.free_type_vars();
                e_vars.remove(X);
                vars.extend(e_vars.drain());
                vars
            }
        }
    }
}