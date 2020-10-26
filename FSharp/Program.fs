// Learn more about F# at http://fsharp.org

open System
open Models
open Utils

let problem1 () =
    let limit = 1000
    let multiplesTargets = [ 3; 5 ]

    multiples limit multiplesTargets
    |> Seq.distinct
    |> Seq.sum

let problem2 () =
    fib ()
    |> Seq.takeWhile (fun i -> i < 4000000)
    |> Seq.filter (fun i -> i % 2 = 0)
    |> Seq.sum

[<EntryPoint>]
let main argv =
    printfn "%A" <| problem2 ()
    0 // return an integer exit code
