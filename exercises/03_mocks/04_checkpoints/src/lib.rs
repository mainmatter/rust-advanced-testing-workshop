//! A loose expectation is causing the test to fail.\
//! Refactor the test in such a way that the issue is reported closer to the source of the problem, then
//! get the test to pass.
use mockall::automock;
use std::collections::HashSet;
use std::error::Error;

pub struct Repository;

impl Repository {
    pub fn new<T: AuthClient>(c: &T, caller_id: usize) -> Self {
        if Permissions::None == c.get_permissions(caller_id) {
            panic!("Caller does not have permissions to create a repository")
        }
        Repository
    }

    pub fn get<T: AuthClient>(&self, c: &T, caller_id: usize, entity_id: usize) -> Entity {
        let permissions = c.get_permissions(caller_id);
        match permissions {
            Permissions::None => panic!("Caller does not have permissions to get an entity"),
            Permissions::Read { ids } | Permissions::Write { ids } => {
                if ids.contains(&entity_id) {
                    Entity
                } else {
                    panic!("Caller does not have permissions to get an entity")
                }
            }
        }
    }
}

#[automock]
pub trait AuthClient {
    fn get_permissions(&self, caller_id: usize) -> Permissions;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Permissions {
    None,
    Read { ids: HashSet<usize> },
    Write { ids: HashSet<usize> },
}

#[derive(Debug, Clone)]
pub struct Entity;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_repository(caller_id: usize, mock_auth_client: &mut MockAuthClient) -> Repository {
        mock_auth_client
            .expect_get_permissions()
            .withf(move |id| *id == caller_id)
            .return_const(Permissions::Read {
                ids: Default::default(),
            });
        let repository = Repository::new(mock_auth_client, caller_id);
        mock_auth_client.checkpoint();
        repository
    }

    #[googletest::gtest]
    fn happy_path() {
        // Setup
        let entity_id: usize = 1;
        let caller_id: usize = 1;
        let mut mock_client = MockAuthClient::new();
        let repository = create_repository(caller_id, &mut mock_client);

        mock_client
            .expect_get_permissions()
            .withf(move |id| *id == caller_id)
            .return_const(Permissions::Read {
                ids: HashSet::from([entity_id]),
            });

        // Act
        repository.get(&mock_client, caller_id, entity_id);
    }
}
