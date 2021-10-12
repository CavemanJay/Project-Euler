{-# LANGUAGE NumericUnderscores #-}

import Control.Monad (join)
import Data.Char (ord)
import Data.List (find, group, permutations, sort, sortBy, union)
import Data.List.Split (splitOn)
import Data.Maybe (fromMaybe)
import System.Directory.Internal.Prelude (fromMaybe)
import Utils
  ( amicable,
    collatzSequence,
    digitProduct,
    digits,
    factorList,
    factorial,
    fibs,
    isPalindrome,
    primeFactors,
    primeFactors',
    primes,
  )

getResourcePath = ("resources/" ++)

problem1 = print $ sum $ filter multiple3or5 [1 .. 999]
  where
    multiple3or5 x = any ((== 0) . mod x) [3, 5]

problem1' = print $ sum $ [3, 6 .. 999] `union` [5, 10 .. 999]

problem1'' = print $ sum [x | x <- [1 .. 999], x `mod` 3 == 0 || x `mod` 5 == 0]

problem2 = print $ sum $ filter even $ tail $ takeWhile (< 4_000_000) fibs

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
  str <- readFile $ getResourcePath "problem13.txt"
  print $ take 10 $ show $ sum $ map (read :: String -> Integer) $ lines str

-- Slow execution but oh well
problem14 = print $ fst $ head $ sortByCount [(x, length $ collatzSequence x) | x <- range]
  where
    range = [1 .. 999_999]
    sortByCount :: [(Int, Int)] -> [(Int, Int)]
    sortByCount = sortBy (\(startA, countA) (startB, countB) -> compare countB countA)

problem16 = print $ sum $ digits $ 2 ^ 1000

problem20 = print $ sum $ digits $ factorial 100

problem21 = print $ sum $ filter amicable [1 .. 9_999]

problem22 = do
  file <- readFile $ getResourcePath "problem22.txt"
  let names = zip [1 ..] (sort $ map unquote $ splitOn "," file)
  print $ sum $map score names
  where
    unquote name = unwords $ map (filter (/= '"')) (words name)
    score :: (Int, [Char]) -> Int
    score name = fst name * sum (map (((* (-1)) . (64 -)) . ord) (snd name))

problem24 = print $ (!! 999_999) $ sort $ map (join . map show) $ permutations [0 .. 9]

problem25 = print $ fst $ fromMaybe (-1, -1) $ firstGreaterThan 1000
  where
    digitCount = length . digits
    firstGreaterThan n = find (\(index, x) -> n <= digitCount x) fibs'
    fibs' = zip [0 ..] fibs

problem29 = print $length $ uniq $ sort $ map (uncurry (^)) $ [(x, y) | x <- range, y <- range]
  where
    range = [2 .. 100]
    uniq = map head . group

problem30 = print $ sum $ take 6 $ filter (\x -> x == digitPowerSum x) [2 ..]
  where
    digitPowerSum = sum . map (^ 5) . digits

problem34 = print $ last $ take 2 $ scanl1 (+) $ filter isDigitFactorial [3..]
  where
    isDigitFactorial n = n == sum (map factorial $ digits n)

main :: IO ()
main = problem34
