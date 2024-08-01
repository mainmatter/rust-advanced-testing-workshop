# HTTP mocking

We have looked at the filesystem and at databases. It's time to turn our attention to another network-driven interaction:
HTTP requests and responses.

## The challenge

Most applications rely on external services to fulfill their purposes.\
Communication with these services usually happens over the network.

Your code can have complex interactions with these dependencies. Depending on the data you send and receive,
your code might go down very different execution paths. The interaction itself might fail in many different ways, which
you must handle appropriately.

> For the purpose of this section, we'll assume that all communication happens over HTTP, but similar techniques
> can be applied to other protocols.

## HTTP mocking

How do you test your code in these conditions?\
At a high-level, you have three options:

1. You run a test instance of the external service that your code can communicate with during the test.
2. You use a library that can intercept HTTP requests and return pre-determined responses.
3. You hide the network dependency behind an abstraction and use a test double rather than the production
   implementation in your tests.

Option #1 (a complete end-to-end test) is the most realistic setup and gives you the highest confidence in your code.
Unfortunately, it's not always feasible: you might not have access to the service, or it might be too expensive to run
an isolated instance for each test (e.g. a deep microservice architecture would require you to run _a lot_ of services since
each service may depend on others).

Option #3 has been explored in the mocking section of the workshop, so let's set it aside for now.

Option #2 is a middle-ground: you're still running the production implementation of your HTTP client, therefore
exercising the whole stack (from your code to the network and back), but you're dodging the complexity of running an actual test instance
of the external service.\
The downside: you need to make sure that your mocked responses are in sync with the real service. If the service changes
its API or behaviour, you need to update your mocks accordingly.

In this section, we'll explore option #2 using [`wiremock`](https://crates.io/crates/wiremock).
