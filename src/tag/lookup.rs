use std::collections::{HashSet, HashMap};
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
        return Err(LookupError::NoChannel);
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
                }
                let package_tree = head_tree.get_path(rel_path.as_path());
                packages_found.insert(rel_path, package_tree.unwrap().id());
            }
        }
    };
    Ok(packages_found)
}
/*
    refs = channel_releases(channel)

    def look_back(name):
        'Iterate through historic releases until we find
        for ref in refs:
            tree = git.tag_dict(ref)['body'].get(name)
            if tree:
                return tree
    git.checkout(refs[0])
    ret_dict = {}
    previous_release = git.tag_dict(refs[0])['body']
    for name, attrs in packages('.').items():
        if name not in previous_release:
            tree = look_back(name)
        else:
            tree = previous_release[name]
        ret_dict[name] = tree
    return ret_dict
    */

pub fn channel_latest (repo: &git2::Repository, channel: &str) -> Result<(), LookupError> {
    let refs = channel_releases(repo, channel).unwrap();
    let latest_tree = repo.revparse_single(&refs[0].1).unwrap();
    println!("{:?}", refs[0].1);
    try!(repo.checkout_tree(&latest_tree, None));
    Ok(())
}
/*
        for pkg_spec in pkg_specs {
            if pkg_spec.contains("/") {
                return Err(ReleaseError::PackagePathDisallowed);
            }
            for tree_entry in target_tree.iter() {
                let tree_name = tree_entry.name().unwrap();
                if tree_name == pkg_spec {
                    let pkg_object = tree_entry.to_object(&repo).unwrap();
                    pkgs.insert(pkg_spec, pkg_object);
                }
            }
        }
*/
