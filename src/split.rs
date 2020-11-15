use alpm::Alpm;
use alpm_utils::{AsTarg, DbListExt, Targ};

/// The "aur mode".
///
/// This means that targets should be interprated as either always from the aur, always from the
/// repos, or coming from either.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    /// Aur only
    Aur,
    /// Repo only
    Repo,
    /// From anywhere
    Any,
}

impl Mode {
    /// Returns true if the mode == Mode::Any
    pub fn is_any(self) -> bool {
        self == Mode::Any
    }

    /// Returns true if the mode == Mode::Aur
    pub fn is_aur(self) -> bool {
        self == Mode::Aur
    }

    /// Returns true if the mode == Mode::Repo
    pub fn is_repo(self) -> bool {
        self == Mode::Repo
    }
}

/// Split a list of repo packages, based on if they apear in the sync database.
///
/// If a package exists in the sync database it added to the repo list. Otherwise it is assumed to
/// be from the aur.
///
/// The return is (repo_pkgs, aur_pkgs)
pub fn split_repo_aur_pkgs<S: AsRef<str> + Clone>(alpm: &Alpm, pkgs: &[S]) -> (Vec<S>, Vec<S>) {
    let mut local = Vec::new();
    let mut aur = Vec::new();

    for pkg in pkgs {
        if alpm.syncdbs().pkg(pkg.as_ref()).is_ok() {
            local.push(pkg.clone());
        } else {
            aur.push(pkg.clone());
        }
    }

    (local, aur)
}

/// The same as `split_repo_aur_pkgs` but takes mode into account.
pub fn split_repo_aur_mode<S: AsRef<str> + Clone>(
    alpm: &Alpm,
    mode: Mode,
    pkgs: &[S],
) -> (Vec<S>, Vec<S>) {
    match mode {
        Mode::Aur => (Vec::new(), pkgs.to_vec()),
        Mode::Repo => (pkgs.to_vec(), Vec::new()),
        Mode::Any => split_repo_aur_pkgs(alpm, pkgs),
    }
}

/// The same as `split_repo_aur_pkgs_mode` but uses targets instead of strings.
///
/// Unlike the previous functions which match on package name. This will take providers and groups
/// into account.
pub fn split_repo_aur_targets<'a, T: AsTarg>(
    alpm: &Alpm,
    mode: Mode,
    targets: &'a [T],
) -> (Vec<Targ<'a>>, Vec<Targ<'a>>) {
    let mut local = Vec::new();
    let mut aur = Vec::new();

    match mode {
        Mode::Aur => return (Vec::new(), targets.iter().map(|t| t.as_targ()).collect()),
        Mode::Repo => return (targets.iter().map(|t| t.as_targ()).collect(), Vec::new()),
        Mode::Any => {
            for targ in targets {
                let targ = targ.as_targ();
                if let Some(repo) = targ.repo {
                    if repo == "aur" {
                        aur.push(targ);
                    } else {
                        local.push(targ);
                    }
                } else if alpm.syncdbs().find_target_satisfier(targ.pkg).is_some()
                    || alpm
                        .syncdbs()
                        .iter()
                        .filter(|db| targ.repo.is_none() || db.name() == targ.repo.unwrap())
                        .any(|db| db.group(targ.pkg).is_ok())
                {
                    local.push(targ);
                } else {
                    aur.push(targ);
                }
            }
        }
    }

    (local, aur)
}
