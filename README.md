# git-tags-rs
An attempt to implement https://github.com/bmcorser/git-tags-py in Rust

## Setup
To define packages, add `.gitattributes` files to your repository
 
## Release logic
required args
 - channel
 - pkgs


## Release data
tag:
  ref: refs/tags/releases/<channel>/<number>
  object: <commit>
  body:
    packages:
      <name>: <tree>
      <name>: <tree>
[note:
  ref: refs/notes/releases
  object: <tag>]
