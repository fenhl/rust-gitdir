use {
    std::path::PathBuf,
    itertools::Itertools as _,
    crate::{
        Host,
        gitdir
    }
};

#[derive(Debug)]
pub enum Error {
    Spec
}

#[derive(Debug, Default)]
pub struct GitHub;

impl Host for GitHub {
    type Error = Error;

    fn repo_dir(&self, repo_spec: &str) -> Result<PathBuf, Error> {
        let (owner, name) = repo_spec.split('/').collect_tuple().ok_or(Error::Spec)?;
        Ok(gitdir().join("github.com").join(owner).join(name))
    }
}
