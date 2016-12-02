Let Bool = forall A. A -> A -> A in
let true: Bool = Lambda A. lambda x: A. lambda y: A. x in
let false: Bool = Lambda A. lambda x: A. lambda y: A. y in
let not: Bool -> Bool = lambda b: Bool. Lambda A. lambda x: A. lambda y: A. b [A] y x in
let if: forall A. Bool -> A -> A -> A = Lambda A. lambda b: Bool. lambda t: A. lambda f: A. b [A] t f in
if [Bool] true false true