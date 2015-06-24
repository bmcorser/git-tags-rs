use std::error::Error;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use glob::glob;
use git2;
use error::LookupError;

pub fn channel_releases<'a> (repo: &'a git2::Repository, channel: &str) -> Result<Vec<(u32, String)>, LookupError> {
    let mut refs = Vec::new();
    let channel_glob = format!("refs/tags/releases/{}/*", channel);
    for reference in repo.references_glob(&channel_glob).unwrap() {
        let ref_name = reference.name().unwrap().to_string();
        let number = ref_name.split("/").last().unwrap().parse::<u32>().unwrap();
        refs.push((number, ref_name));
    }
    if refs.len() == 0 {
        return Err(LookupError::EmptyChannel);
    }
    refs.sort_by(|&(a, _), &(b, _)| b.cmp(&a));
    Ok(refs)
}

pub fn packages (repo: &git2::Repository) -> Result<HashMap<PathBuf, git2::Oid>, LookupError> {
    let mut packages_found = HashMap::new();
    let head = repo.revparse_single("HEAD").unwrap();
    let head_peeled = head.peel(git2::ObjectType::Tree).unwrap();
    let head_tree = head_peeled.as_tree().unwrap();
    for top_entry in fs::read_dir(repo.workdir().unwrap()).unwrap() {
        let top_entry = top_entry.unwrap().path();
        // println!("{:?}", top_entry);
        if !top_entry.ends_with(".git") {  // don’t look inside there!
            let top_entry = top_entry.to_str().unwrap();
            let package_paths = glob(&format!("{}/**/.package", &top_entry)).unwrap();
            for package_path in package_paths {
                let mut path = package_path.unwrap();
                path.pop();
                let workdir = repo.workdir().unwrap();
                let rel_path = path.relative_from(workdir).unwrap().to_path_buf();
                for path in packages_found.keys() {
                    if rel_path.starts_with(path) {
                        return Err(LookupError::NestingError);
                    }
                }
                let package_tree = head_tree.get_path(rel_path.as_path());
                packages_found.insert(rel_path, package_tree.unwrap().id());
            }
        }
    };
    Ok(packages_found)
}

pub fn channel_latest (repo: &git2::Repository, channel: &str) -> Result<(u32, HashMap<PathBuf, git2::Oid>), LookupError> {
    let refs = try!(channel_releases(repo, channel));
    let latest_tree = repo.revparse_single(&refs[0].1).unwrap();
    try!(repo.reset(&latest_tree, git2::ResetType::Hard, None));
    let ret_val = (refs[0].0, packages(&repo).unwrap());
    try!(repo.reset(&repo.revparse_single("HEAD").unwrap(),
                    git2::ResetType::Hard, None));
    Ok(ret_val)
}

pub fn channel_release (repo: &git2::Repository, channel: &str, number: u32) -> Result<(u32, HashMap<PathBuf, git2::Oid>), LookupError> {
    let reference = format!("refs/tags/releases/{}/{}", channel, number);
    let release_tree = match repo.revparse_single(&reference) {
        Ok(release_tree) => release_tree,
        Err(err)    => {
            println!("Git said: {:?}", err.description());
            println!("No release numer {:?} in channel {:?}", number, channel);
            return Err(LookupError::NotFound);
        }
    };
    try!(repo.reset(&release_tree, git2::ResetType::Hard, None));
    let ret_val = (number, packages(&repo).unwrap());
    try!(repo.reset(&repo.revparse_single("HEAD").unwrap(),
                    git2::ResetType::Hard, None));
    Ok(ret_val)
}
