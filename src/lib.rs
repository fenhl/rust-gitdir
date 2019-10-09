use {
    std::{
        convert::TryFrom,
        env,
        path::{
            Path,
            PathBuf
        }
    },
    dirs::home_dir
};
pub use host::github::GitHub;

pub mod host;

pub trait Host {
    type Error;

    fn repo<'a, 'b>(&'a self, repo_spec: &'b str) -> Repo<'a, 'b, Self> {
        Repo {
            repo_spec,
            host: self
        }
    }

    fn repo_dir(&self, repo_spec: &str) -> Result<PathBuf, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Repo<'a, 'b, H: Host + ?Sized> {
    pub host: &'a H,
    pub repo_spec: &'b str
}

impl<H: Host + ?Sized> Repo<'_, '_, H> {
    pub fn master(&self) -> Result<PathBuf, H::Error> {
        Ok(self.host.repo_dir(self.repo_spec)?.join("master"))
    }
}

impl<H: Host + ?Sized> TryFrom<Repo<'_, '_, H>> for PathBuf {
    type Error = H::Error;

    fn try_from(repo: Repo<H>) -> Result<PathBuf, H::Error> {
        repo.host.repo_dir(repo.repo_spec)
    }
}

pub fn gitdir() -> PathBuf {
    if let Some(path) = env::var_os("GITDIR") {
        path.into()
    } else {
        let local = local_gitdir();
        let global = global_gitdir();
        if local.as_ref().map_or(false, |local| local.exists()) && !global.exists() { local.unwrap() } else { global.into() } //TODO check permissions
    }
}

pub fn global_gitdir() -> &'static Path {
    Path::new("/opt/git")
}

pub fn local_gitdir() -> Option<PathBuf> {
    home_dir().map(|home| home.join("git"))
}
