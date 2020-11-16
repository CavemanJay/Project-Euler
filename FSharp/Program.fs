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


let problem10 () =
    generateNPrimes 1000000000
    |> Seq.takeWhile (fun i -> i < 2000000L)
    |> Seq.sum


let problem12 () =
    Seq.initInfinite (fun i -> nthTriangularNumber i)
    |> Seq.tail
    |> Seq.find (fun i -> (i |> int64 |> factors |> List.ofSeq).Length > 500)


let problem13 () =
    getResource "Problem 13.txt"
    |> fun str -> str.Split()
    |> Seq.filter (fun i -> i <> "")
    |> Seq.map (bigint.Parse)
    |> Seq.sum
    |> string
    |> fun str -> str.[0..9]


let problem14 () =
    [ 1L .. 1000000L ]
    |> Seq.map (fun i -> (i, collatz i))
    |> Seq.maxBy (fun (_, chainCount) -> chainCount)
    |> fun (start, chainCount) -> start


let problem16 () =
    bigint.Pow(bigint.Parse("2"), 1000)
    |> string
    |> sumOfDigits


let problem20 () =
    100
    |> bigint
    |> factorial
    |> string
    |> sumOfDigits


let problem21 () =
    let d n = properDivisors n |> Seq.sum

    let range = [ 2L .. 9999L ]
    let mutable pairs = [] |> seq

    for a in range do
        let da = d a
        let db = d da
        if a = db && a <> da then pairs <- Seq.append pairs [ da; db ]

    pairs |> set |> Seq.sum


let problem22 () =
    let getScore (name: string) index =
        (name |> Seq.map (fun c -> (int c) - 64) |> Seq.sum)
        * (index + 1)

    getResource "Problem 22.txt"
    |> fun str -> str.Split(',')
    |> Seq.map (fun str -> str.Replace("\"", ""))
    |> Seq.sort
    |> Seq.mapi (fun index name -> getScore name index)
    |> Seq.sum

type Designation =
    | Deficient of int64
    | Abundant of int64
    | Perfect of int64

//let problem23 () =
//    let getDesignation n =
//        match properDivisors n |> Seq.sum with
//        | i when i > n -> Abundant n
//        | i when i < n -> Deficient n
//        | i -> Perfect n

//    let canBeWritten n =
//        let range = [ 1 .. n - 1 ]

//        for i in range do
//            ()

//    [ 1L .. 28123L ]
//    |> Seq.map getDesignation
//    |> Seq.choose (function | )


let problem24 () =
    // https://stackoverflow.com/a/2184129
    let rec insertions x =
        function
        | [] -> [ [ x ] ]
        | (y :: ys) as l ->
            (x :: l)
            :: (List.map (fun x -> y :: x) (insertions x ys))

    let rec permutations =
        function
        | [] -> seq [ [] ]
        | x :: xs -> Seq.concat (Seq.map (insertions x) (permutations xs))

    "0123456789"
    |> List.ofSeq
    |> permutations
    |> Seq.map (fun i -> i |> Seq.map string |> String.concat "")
    |> Seq.sort
    |> Seq.item 999999

[<EntryPoint>]
let main argv =
    printfn "%A" <| problem24 ()
    0 // return an integer exit code
