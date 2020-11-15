// Learn more about F# at http://fsharp.org

open System
open Models
open Utils
open Operators

let problem1 () =
    let limit = 1000
    let multiplesTargets = [ 3; 5 ]

    multiples limit multiplesTargets
    |> Seq.distinct
    |> Seq.sum

let problem2 () =
    ()
    |> fib
    |> Seq.takeWhile (fun i -> i < 4000000)
    |> Seq.filter (fun i -> i % 2 = 0)
    |> Seq.sum

let problem3 () =
    let target = 600851475143L

    target
    |= (fun _ -> printfn "Retrieving factors...")
    |> factors
    |= (fun _ -> printfn "Filtering for primes...")
    |> Seq.filter isPrime
    |= (fun _ -> printfn "Finding maximum...")
    |> Seq.max

let problem4 () =
    let generateProducts limit bottom =
        let range = [ bottom .. limit ]

        seq {
            for i in range do
                for j in range do
                    yield i * j
        }


    100
    |> generateProducts 999
    |> Seq.filter isPalindrome
    |> Seq.max


[<EntryPoint>]
let main argv =
    printfn "%A" <| problem4 ()
    0 // return an integer exit code
