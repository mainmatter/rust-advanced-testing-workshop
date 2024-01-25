# Assertion infrastructure

As you've seen in the previous exercise, you can get pretty nice test failure messages with the standard library's
assertions **if** you take the time to write a custom message.
That additional friction is a problem, though.  

If you don't bother to write a custom message, you'll get a generic error that doesn't help you understand what went wrong. 
It'll take you longer to fix tests.

If you choose to bother, you don't want to write the same custom message over and over again. You want to write it once and reuse it.
You end up writing a custom assertion function, like we did in the previous exercise.  
But you aren't working on this project alone. You have a team! You now need to teach your team that this custom assertion 
function exists if you want to have a consistent testing style across your codebase.  
Congrats, you've just written your own assertion library!

## Invest where it matters

Don't get me wrong: you should write custom assertions.  
Once your project gets complex enough, you will **have to** write your own matchers. 
They'll be bespoke to your domain and they'll help you write tests that are easy to read and maintain.

But that's a tiny fraction of the assertions you'll write.  
For all the generic stuff, the one that stays the same across projects, you don't want to take over the burden 
of writing and maintaining your own assertion library.  
In that area, you want to **standardise** on an existing library that's well maintained and has a large community. If 
you do that, you'll be able to reuse your knowledge across projects and you'll be able to find help online when you need it.
You can always choose to contribute to the library if you find a bug or a missing feature.

## `googletest`

There's a few options when it comes to assertion libraries for Rust.  
We'll use [`googletest`](https://crates.io/crates/googletest) in this workshop.  
It's a Rust port of the famous [GoogleTest C++ testing library](https://google.github.io/googletest/).  
It comes, out of the box, with a rich set of _matchers_ and a nice way to write custom ones. It also includes 
a few useful macros for more complex testing scenarios—we'll explore them in the coming exercises.

## Exercise

Before digging any deeper, let's get familiar with the basics of `googletest` by rewriting the tests 
from the previous exercise.