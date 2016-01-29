{-# LANGUAGE GADTs #-}
module Main where

main = putStrLn $ "Tiny expression : " ++ (eval simpleExpr)

simpleExpr :: Expr Bool
simpleExpr = (Lt (Add (IV "a") (IC 1)) (Add (IV "b") (IC 120)))

-- this expression is broken, and dissallowed by the typechecker
-- brokenExpr = (Add (Lt (IV "a") (IV "b")) (BV True))

data Expr a where
    IV :: String -> Expr Int
    IC :: Int -> Expr Int
    BV :: String -> Expr Int
    BC :: Bool -> Expr Bool
    Add :: Expr Int -> Expr Int -> Expr Int
    Lt :: Expr Int -> Expr Int -> Expr Bool

eval :: Expr a -> String
eval (IV v) = v
eval (IC c) = show c
eval (BV v) = v
eval (BC c) = show c
eval (Add lhs rhs) = "("++(eval lhs) ++"+"++(eval rhs)++")"
eval (Lt lhs rhs) = "("++(eval lhs) ++"<"++(eval rhs)++")"

