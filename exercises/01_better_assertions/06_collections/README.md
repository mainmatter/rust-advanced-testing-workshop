# `googletest` - Collections

We close our tour of `googletest`'s built-in matchers with a look at specialised matchers for collections.  

`googletest` really shines with collections. The matchers are very expressive and can be combined in powerful ways.
Failure messages are also extremely helpful, showing the actual values and highlighting the differences.  
Achieving the same level of helpfulness with `assert!` would require a lot of boilerplate!

## Exercise

Replace the `assert!` calls with the corresponding `googletest` matchers.  