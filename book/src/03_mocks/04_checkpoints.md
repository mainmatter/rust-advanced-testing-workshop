# Checkpoints

The test may need to use your mock object as part of its setup as well as a dependency of the specific
code path under test.

For example:

```rust
pub struct Repository(/* ... */);

impl Repository {
    pub fn new<T: UserProvider>(up: T) -> Self {
        // ...
    }

    pub fn query<T: UserProvider>(&self, id: u32, up: T) -> Option<Entity> {
        // ...
    }
}
```

If you're mocking `UserProvider` and you want to test `Repository::query`, you'll need to use the mock
for calling `Repository::new` first.

## Expectations can leak

To get `Repository::new` to behave as expected, you'll need to set up some expectations on `MockUserProvider`.\
You'll also need to set up expectations on `MockUserProvider` for `Repository::query` to behave as expected.

There's a risk that the expectations you set up for `Repository::new` will **leak** into `Repository::query`:
they'll be executed when they shouldn't be, leading to confusing errors in your tests.\
This can happen, in particular, when the code in `Repository::new` changes and stops performing one of
the calls you set up expectations for.

## Checkpoints

To prevent this from happening, you can use two different instances of `MockUserProvider` for those calls.\
Alternatively, you can rely on **checkpoints**.\
A checkpoint is a way of saying "Panic unless all expectations up to this point to have been met".

In this example, you can use a checkpoint to ensure that the expectations for `Repository::new` are met
before you start setting up expectations for `Repository::query`.

```rust
#[test]
fn test_repository_query() {
    let mut mock = MockUserProvider::new();
    let mut repo = setup_repository(&mut mock);

    // Set up expectations for Repository::query
    // [...]

    // Call Repository::query
    // [...]
}

fn setup_repository(mock: &mut MockUserProvider) -> Repository {
    // Arrange
    mock.expect_is_authenticated()
        .returning(|_| true);
    // [...]

    // Act
    let repository = Repository::new(mock);

    // Verify that all expectations up to the checkpoint have been met
    mock.checkpoint();

    repository
}
```

If expectations are not met at the checkpoint, it will panic.\
If they are met, the test will continue and all expectations will be reset.
