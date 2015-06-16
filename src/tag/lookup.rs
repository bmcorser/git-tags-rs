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
