module Utils where

import Control.Monad (join)
import Data.Bits
import Data.Char (digitToInt, ord)
import Data.List
import Data.Maybe

-- fibs = 0 : 1 : next fibs
--   where
--     next (a : t@(b : _)) = (a + b) : next t
fibs = unfoldr (\(a, b) -> Just (a, (b, a + b))) (0, 1)

fib :: Int -> Integer
fib n =
  snd . foldl_ fib_ (1, 0) . dropWhile not $
    [testBit n k | k <- let s = finiteBitSize n in [s -1, s -2 .. 0]]
  where
    fib_ (f, g) p
      | p = (f * (f + 2 * g), ss)
      | otherwise = (ss, g * (2 * f - g))
      where
        ss = f * f + g * g
    foldl_ = foldl' -- '

isFactorOf x n = n `mod` x == 0

factorList n = filter (`isFactorOf` n) [1 .. n `div` 2]

primeFactors :: (Integral a) => a -> [a]
primeFactors = unfoldr (\n -> listToMaybe [(x, div n x) | x <- [2 .. n], mod n x == 0])

isPrime :: (Integral a) => a -> Bool
isPrime n = n > 1 && head (primeFactors n) == n

isPalindrome x = reverse (show x) == show x

isDivisibleByRange :: Integral a => [a] -> a -> Bool
isDivisibleByRange range x = all (\n -> x `mod` n == 0) range

-- isDivisibleByRange' :: Integral a => [a] -> a -> Bool
-- isDivisibleByRange'  range x= all (\x->x) $ map (\n -> x `mod` n == 0) range
primeFactors' n = factor n primes

primes = 2 : filter (null . tail . primeFactors') [3, 5 ..]

factor n (p : ps)
  | p * p > n = [n]
  | n `mod` p == 0 = p : factor (n `div` p) (p : ps)
  | otherwise = factor n ps

digitProduct = product . map digitToInt

collatz 1 = 1
collatz n
  | even n = n `div` 2
  | otherwise = 3 * n + 1

-- https://adamo.wordpress.com/2018/03/17/a-collatz-sequence-in-haskell/
collatzSequence 1 = [1]
collatzSequence n = n : collatzSequence (collatz n)

-- (:"") Converts a char to a string
digits :: (Integral a, Show a) => a -> [Int]
digits n = map (read . (: "")) $ show n :: [Int]

undigits :: (Integral a, Show a) => [a] -> Int
undigits ns = read $ join (map show ns) :: Int

factorial n = product [1 .. n]

amicable a = a == db && a /= da
  where
    d = sum . factorList
    da = d a
    db = d da

-- https://stackoverflow.com/a/11873846
rotations xs = init (zipWith (++) (tails xs) (inits xs))

circle = map undigits . rotations . digits

unquote s = unwords $ map (filter (/= '"')) (words s)

nthTriangle n = n * (n + 1) `div` 2

alphabeticalScore = (* (-1)) . (64 -) . ord