-- Type Signature
main :: IO ()

main = do  
    putStrLn "Hello, lets create your world.  What's your name?"  
    name <- getLine  
    putStrLn ("Hello " ++ name ++ "'s World!" ++ " It rocks!")  
