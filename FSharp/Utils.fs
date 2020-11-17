module Utils

open System
open System.IO
open System.Reflection

/// Returns the multiples of the specified vals below the limit
let multiples limit ([<ParamArray>] vals: int seq) =
    seq {
        for i in [ 2 .. limit - 1 ] do
            for num in vals do
                if i % num = 0 then yield i
    }

/// Fibonacci sequence
let fib () =
    Seq.unfold (fun (a, b) -> Some(a + b, (b, a + b))) (bigint.Zero, bigint.One)

/// Tests whether a number is prime
let isPrime (n: int64) =
    let limit =
        lazy (Math.Sqrt(float n) |> Math.Ceiling |> int64)

    match n with
    | 1L -> false
    | 2L -> true
    | x ->
        if x % 2L = 0L then
            false
        else
            [ 2L .. limit.Value ]
            |> Seq.forall (fun i -> n % i <> 0L)

let isDivisible a b = a % b = 0L


// Converted from the python algorithm at: https://stackoverflow.com/a/19578818
let factors (n: int64) =
    let step = if n % 2L = 1L then 2L else 1L

    seq {
        for i in [ 1L .. step .. (Math.Sqrt(n |> float) |> int64) + 1L ] do
            if n % i = 0L then yield [ i; n / i ]
    }
    |> Set.ofSeq
    |> Seq.concat

let properDivisors n =
    factors n |> Seq.filter (fun i -> i <> n)

let reverse (str: string) =
    str
    |> Seq.toList
    |> List.rev
    |> List.map (fun c -> c |> string)
    |> Seq.ofList
    |> String.concat ""


//let isPalindrome (n: int64) = n |> string = (n |> string |> reverse)
let isPalindrome (n: bigint) = n |> string = (n |> string |> reverse)

let isPythagoreanTriplet (a: int) (b: int) (c: int) =
    (Math.Pow(a |> float, 2.0) |> int)
    + (Math.Pow(b |> float, 2.0) |> int) = (Math.Pow(c |> float, 2.0) |> int)


let generateNPrimes n =
    seq {
        let mutable count = 0

        let mutable i = 2L

        while count <= n do
            if i |> int64 |> isPrime then
                count <- count + 1
                yield i

            i <- i + 1L
    }

let primes =
    Seq.initInfinite (fun i -> i)
    |> Seq.map int64
    |> Seq.filter isPrime


let nthTriangularNumber n = [ 1 .. n ] |> List.sum


/// Calculates the number of links in the collatz chain with n as the starting point
let rec collatz (n: int64) =
    if n = 1L then
        1
    else
        match n % 2L with
        | 0L -> 1 + collatz (n / 2L)
        | 1L -> 1 + collatz (3L * n + 1L)


let rec factorial (n: bigint) =
    if n = bigint.Zero then bigint.One else n * factorial (n - bigint.One)


/// Returns a list of the integer digits in a number
let getDigits (n: string) =
    n
    |> Seq.map (string)
    |> Seq.map (int)
    |> List.ofSeq

let sumOfDigits (n: string) = n |> getDigits |> Seq.sum

let permutations list =
    // https://stackoverflow.com/a/2184129
    let rec insertions x =
        function
        | [] -> [ [ x ] ]
        | (y :: ys) as l ->
            (x :: l)
            :: (List.map (fun x -> y :: x) (insertions x ys))

    let rec permute =
        function
        | [] -> seq [ [] ]
        | x :: xs -> Seq.concat (Seq.map (insertions x) (permute xs))

    permute list

let truncatations num ltr =
    let cutLeft n =
        n
        |> string
        |> getDigits
        |> Seq.tail
        |> Seq.map string
        |> String.concat ""
        |> int

    let cutRight n =
        n
        |> string
        |> getDigits
        |> Seq.map string
        |> String.concat ""
        |> int
        |> fun i -> i / 10

    seq {
        yield num

        let mutable count = 1
        let mutable n = num

        while count < num.ToString().Length do
            match n with
            | _ when n < 10 -> ()
            | _ when ltr -> n <- cutLeft n
            | _ -> n <- cutRight n

            count <- count + 1
            yield n
    }

let circularPermutations (str: string) =
    let digitCount = str.Length

    let powered =
        Math.Pow(10.0, float digitCount - 1.0) |> int64

    let mutable num = int64 str

    seq {
        for i = 0 to digitCount - 1 do
            let leftMost = num / powered

            let rotation =
                ((num * 10L) + leftMost)
                - (leftMost * powered * 10L)

            yield rotation
            num <- rotation
    }

let getResource fileName =
    let assembly = Assembly.GetExecutingAssembly()


    let fqn = sprintf "FSharp.Resources.%s" fileName
    use resourceStream = assembly.GetManifestResourceStream(fqn)

    use reader = new StreamReader(resourceStream)
    reader.ReadToEnd()
