# Outro

The patterns (and tools) you've learned in this section should help you write more robust tests
for code that interacts with the filesystem.  

## When everything else fails

Nonetheless, they are not a silver bullet: you might not be in control of the code you're testing (e.g. third-party libraries),
or it might be too expensive to refactor it to make it more testable.

In those cases, you can take a more radical approach to isolation: run each test in a separate process 
and set their working directory to a temporary directory (created via `tempfile::TempDir`).  
If you want to go down that route, check out [`cargo-nextest`](https://nexte.st/): it runs your tests as 
isolated processes by default, and it's best suited for this kind of workflow.
