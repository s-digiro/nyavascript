# To Do
[ ] Consider returning data type errors rather than evaluating errors to nil
[ ] Add tests to make sure McCarthy scope contains what we expect it to
[ ] Pass env to macro
[ ] Add line and column to errors where applicable
[ ] Passing lambdas to other lambdas is a lil weird... figure it out
[ ] McCarthy things need to be implemented
    [*] Car
    [*] Cons
    [*] Cdr
    [*] Equal
    [*] quote
    [ ] Maybe more
    [*] McCarthy context??? as a type of Rust context?
    [ ] Make sure all McCarthy things are added
[ ] Lib user defined Rust Contexts
[ ] Rust context of lib that I find useful
    [ ] Like print
    [ ] Add/Subtract
    [ ] Stuff that isn't pure McCarthy
[ ] Figure out the difference and standard practices between e2e tests and lib unit tests
[ ] Syntax parsing is a wreck, refactor to make it easier to read
[ ] Semantic parsing is probably redundant and the work being done there is
    probably better off being done in syntax parsing
[ ] Garbage collector
[ ] Evaluate List should be refactored into more functions so its easier to read
[ ] Allow multiple lists in root, without tacking on a root list as a new
    function. Maybe as a vector of refvals
[ ] Maybe strings should be lists of chars? Or maybe permanently quoted symbols?
    Pick one
[ ] List macro
[ ] Label macro
[ ] let macro
[ ] arbitrary cdaddr function
[ ] Maybe mccarthy functions should be "specials", separate from macros?
    decide.
[ ] From SX for SXRef
[ ] From inner for SXRef
[ ] From inner for SX
[ ] Better toString for Function
[ ] Better toString for Macro
[ ] Find a way to implement PartialEq on RustFunction
[ ] Find a way to implement PartialEq on RustMacro
[*] Move closures in McCarthyScope to their own functions?
