module Utils

open System

/// Returns the multiples of the specified vals below the limit
let multiples limit ([<ParamArray>] vals: int seq) =
    seq {
        for i in [ 2 .. limit - 1 ] do
            for num in vals do
                if i % num = 0 then yield i
    }
