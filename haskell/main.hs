{-# LANGUAGE NumericUnderscores #-}

import Data.List (sortBy, union)
import Utils
  ( collatzSequence,
    digitProduct,
    fibs,
    isPalindrome,
    primeFactors,
    primeFactors',
    primes,
  )

problem1 = print $ sum $ filter multiple3or5 [1 .. 999]
  where
    multiple3or5 x = any ((== 0) . mod x) [3, 5]

problem1' = print $ sum $ [3, 6 .. 999] `union` [5, 10 .. 999]

problem1'' = print $ sum [x | x <- [1 .. 999], x `mod` 3 == 0 || x `mod` 5 == 0]

problem2 = print $ sum $ filter even $ tail $ takeWhile (<= 4_000_000) fibs

problem3 = print $ last $ primeFactors 600851475143

problem3' = print $ last (primeFactors' 600851475143)

problem4 = print $ maximum $ filter isPalindrome $ [x * y | x <- range, y <- range]
  where
    range = [100 .. 999]

problem5 = print $ foldr1 lcm [1 .. 20]

problem6 = print $ squareOfSum - sumOfSquares
  where
    limit = 100
    sumOfSquares = sum $ map (^ 2) [1 .. limit]
    squareOfSum = sum [1 .. limit] ^ 2

problem7 = print $ primes !! 10_000

problem10 = print $ sum $ takeWhile (< 2_000_000) primes

problem13 = do
  str <- readFile "problem13.txt"
  print $ take 10 $ show $ sum $ map (read :: String -> Integer) $ lines str

-- Slow execution but oh well
problem14 = print $ fst $ head $ sortByCount [(x, length $ collatzSequence x) | x <- range]
  where
    range = [1 .. 999_999]
    sortByCount :: [(Int, Int)] -> [(Int, Int)]
    sortByCount = sortBy (\(startA, countA) (startB, countB) -> compare countB countA)

main :: IO ()
main = problem14
