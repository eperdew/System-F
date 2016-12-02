use systemf::Type::*;

#[test]
fn test_eq1() {
    let t1 = Var(String::from("X"));
    let t2 = Var(String::from("Y"));

    assert_eq!(t1, t1);
    assert_eq!(t2, t2);
    assert!(t1 != t2);

    let t3 = Fun(Box::new(t1.clone()), Box::new(t2.clone()));
    let t4 = Fun(Box::new(t2.clone()), Box::new(t1.clone()));

    assert_eq!(t3, t3);
    assert_eq!(t4, t4);
    assert!(t3 != t4);

    let t5 = Forall(String::from("X"),Box::new(t1.clone()));
    let t6 = Forall(String::from("Y"),Box::new(t2.clone()));

    assert_eq!(t5, t6);

    assert!(t1 != t3);
    assert!(t1 != t5);
    assert!(t3 != t5);
}

#[test]
fn test_eq2() {
    let t1 = Forall(String::from("X"),
                Box::new(Forall(String::from("Y"),
                Box::new(Var(String::from("X"))))));
    let t2 = Forall(String::from("Y"),
                Box::new(Forall(String::from("X"),
                Box::new(Var(String::from("Y"))))));

    assert_eq!(t1, t2);

    let t3 = Forall(String::from("X"),
                Box::new(Forall(String::from("Y"),
                Box::new(Var(String::from("Y"))))));

    let t4 = Forall(String::from("Y"),
                Box::new(Forall(String::from("X"),
                Box::new(Var(String::from("X"))))));

    assert_eq!(t3, t4);                

    let t5 = Forall(String::from("Y"),
                Box::new(Forall(String::from("Y"),
                Box::new(Var(String::from("Y"))))));

    let t6 = Forall(String::from("X"),
                Box::new(Forall(String::from("X"),
                Box::new(Var(String::from("X"))))));

    assert_eq!(t5, t6);
}