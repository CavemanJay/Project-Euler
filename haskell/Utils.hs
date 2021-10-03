module Utils where

import Data.Bits
import Data.Char (digitToInt)
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

primeFactors = unfoldr (\n -> listToMaybe [(x, div n x) | x <- [2 .. n], mod n x == 0])

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