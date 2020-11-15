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

    target |> factors |> Seq.filter isPrime |> Seq.max

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

let problem5 () =
    let checkRange range n =
        if n % 5000000 = 0 then printfn "Testing: %d" n
        range |> Seq.forall (n |> int64 |> isDivisible)

    let range =
        [ 1 .. 20 ]
        |> List.map (fun x -> x |> int64)
        |> Seq.ofList

    // Generate an infinite list starting at 1
    Seq.initInfinite (fun i -> i + 1)
    // Create a list of every number that is not divisible by every number in the range
    |> Seq.takeWhile (fun i -> checkRange range i |> not)
    |> List.ofSeq
    // The answer is the last element + 1
    |> List.last
    |> fun i -> i + 1


let problem6 () =
    let sumOfSquares range =
        List.fold (fun acc current -> acc + Math.Pow(current, 2.0)) 0.0 range
        |> int

    let squareOfSum range =
        range
        |> List.sum
        |> fun x -> Math.Pow(x, 2.0) |> int


    let range = [ 1.0 .. 100.0 ]
    squareOfSum range - sumOfSquares range

let problem7 () =
    let generateNPrimes limit =
        seq {
            let mutable count = 0

            let mutable i = 2

            while count <= limit do
                if i |> int64 |> isPrime then
                    count <- count + 1
                    yield i

                i <- i + 1
        }

    generateNPrimes 100000000 |> Seq.item 10000


// Not complete
let problem8 () =
    let multiply nums =
        nums
        |> Seq.fold (fun acc current -> acc * current) 1

    let getProducts numDigits (number: string) =

        seq {
            for i = 0 to number.Length - numDigits + 1 do
                number.[i..i + numDigits - 1]
                |> List.ofSeq
                |> Seq.map (fun i -> i |> string |> int)
                |> fun i -> (i |> List.ofSeq, i |> multiply)
        }

    let number =
        (getResource "Problem 8.txt")
            .Replace("\n", "")
            .Replace("\r", "")

    number
    |> getProducts 13
    |> Seq.maxBy (fun (_, product) -> product)


let problem9 () =
    let limit = 1000000

    seq {
        for c = 5 to limit do
            for b = 1 to c do
                for a = 1 to b do
                    if a + b + c = 1000 && isPythagoreanTriplet a b c
                    then ([ a; b; c ], a * b * c)
    }
    |> Seq.take 1
    |> Seq.head


[<EntryPoint>]
let main argv =
    printfn "%A" <| problem9 ()
    0 // return an integer exit code
