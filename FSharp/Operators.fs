module Operators

/// Tap/tee operator
let (|=) x f =
    f x
    x
