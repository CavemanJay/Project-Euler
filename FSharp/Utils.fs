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
    Seq.unfold (fun (a, b) -> Some(a + b, (b, a + b))) (0, 1)

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

/// Yields the factors of a number
let factors (n: int64) =
    let limit = n |> float |> Math.Sqrt |> int64

    [ 1L .. limit ] |> Seq.filter (isDivisible n)

let reverse (str: string) =
    str
    |> Seq.toList
    |> List.rev
    |> List.map (fun c -> c |> string)
    |> Seq.ofList
    |> String.concat ""


let isPalindrome (n: int) = n |> string = (n |> string |> reverse)

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


let getResource fileName =
    let assembly = Assembly.GetExecutingAssembly()


    let fqn = sprintf "FSharp.Resources.%s" fileName
    use resourceStream = assembly.GetManifestResourceStream(fqn)

    use reader = new StreamReader(resourceStream)
    reader.ReadToEnd()
