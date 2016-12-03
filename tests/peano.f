Let Nat = forall A. (A -> A) -> A -> A in
let Z: Nat = Lambda A. lambda f: A -> A. lambda x: A. x in
let S: Nat -> Nat = lambda n: Nat. Lambda A. lambda f: A -> A. lambda x: A. f (n [A] f x) in
let add: Nat -> Nat -> Nat = lambda n: Nat. lambda m: Nat. n [Nat] S m in
let mult: Nat -> Nat -> Nat = lambda n: Nat. lambda m: Nat. n [Nat] (add m) Z in
let two: Nat = mult (S Z) (S (S Z)) in
two