Let Nat = forall A . (A -> A) -> A -> A in
Let idT = forall A . A -> A in
let id : idT = Lambda A . lambda x : A . x in
let S : Nat -> Nat = 
lambda n : Nat . 
    Lambda A . lambda f : A -> A . lambda x : A . f (n [A] f x) in
let zero : Nat = Lambda A . lambda f : A -> A . lambda x : A . x in
zero [idT] (id [idT -> idT]) (id [idT])