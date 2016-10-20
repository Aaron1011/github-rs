//! Trait definition related to Repositories on Github

use requests::*;
use github::Client;
use error::*;
use json::{Repo, RepoCreate};
use serde_json;

/// Trait definition for endpoints grouped under `Repositories` in the Github API specification
pub trait Repos {
    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /user/repos
    /// ### Description
    /// Creates a new repo for the authenticated user and returns the new `Repo`s stats
    fn post_user_repos(&self, repo: RepoCreate) -> Result<Repo>;
}

/// Implementation of the Repos trait for the `Client`
impl Repos for Client {
    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /user/repos
    /// ### Description
    /// Creates a new repo for the authenticated user and returns the new `Repo`s stats
    fn post_user_repos(&self, repo: RepoCreate) -> Result<Repo> {
        let url = "https://api.github.com/user/repos";
        let res = try!(post(url,
                            self.headers.clone(),
                            try!(serde_json::to_string(&repo))));
        try_serde!(serde_json::from_str(&res))
    }
}
