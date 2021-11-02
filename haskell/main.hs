{-# LANGUAGE NumericUnderscores #-}

import Control.Monad (join)
import Data.Char (ord)
import Data.List (find, group, inits, permutations, sort, sortBy, union)
import Data.List.Split (splitOn)
import Data.Maybe (fromMaybe)
import System.Directory.Internal.Prelude (fromMaybe)
import Text.Printf (printf)
import Utils
  ( alphabeticalScore,
    amicable,
    circle,
    collatzSequence,
    digitProduct,
    digits,
    factorList,
    factorial,
    fibs,
    isPalindrome,
    isPrime,
    nthTriangle,
    primeFactors,
    primeFactors',
    primes,
    unquote,
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
  print $ sum $ map score names
  where
    score :: (Int, [Char]) -> Int
    score s = fst s * sum (map alphabeticalScore (snd s))

problem24 = print $ (!! 999_999) $ sort $ map (join . map show) $ permutations [0 .. 9]

problem25 = print $ fst $ fromMaybe (-1, -1) $ firstGreaterThan 1000
  where
    digitCount = length . digits
    firstGreaterThan n = find (\(index, x) -> n <= digitCount x) fibs'
    fibs' = zip [0 ..] fibs

problem29 = print $ length $ uniq $ sort $ map (uncurry (^)) $ [(x, y) | x <- range, y <- range]
  where
    range = [2 .. 100]
    uniq = map head . group

problem30 = print $ sum $ take 6 $ filter (\x -> x == digitPowerSum x) [2 ..]
  where
    digitPowerSum = sum . map (^ 5) . digits

problem34 = print $ last $ take 2 $ scanl1 (+) $ filter isDigitFactorial [3 ..]
  where
    isDigitFactorial n = n == sum (map factorial $ digits n)

problem36 = print $ sum $ filter isDoubleBasePalindrome [1 .. 1_000_000]
  where
    bin :: Int -> [Char]
    bin n = printf "%b" n
    isDoubleBasePalindrome n = isPalindrome (bin n) && isPalindrome n

problem37 = print $ sum $ take 11 $ filter isTruncatablePrime [9, 11 ..]
  where
    rightTruncate :: (Integral a, Show a, Read a) => a -> [a]
    rightTruncate = map read . scanl1 (++) . map show . digits
    leftTruncate :: (Integral a, Show a, Read a) => a -> [a]
    leftTruncate = map (read . reverse) . tail . inits . reverse . show
    isTruncatablePrime n = all isPrime (rightTruncate n) && all isPrime (leftTruncate n)

problem42 = do
  file <- readFile $ getResourcePath "problem42.txt"
  let words = map unquote $ splitOn "," file
  print $ length $ filter (`elem` triangles) $ map wordScore words
  where
    wordScore = sum . map alphabeticalScore
    triangles = map nthTriangle [1 .. 100]

problem48 = print $ join $ map show $ reverse $ take 10 $ reverse $ digits $ sum $ map (\x -> x ^ x) [1 .. 1000]

main :: IO ()
main = problem48
