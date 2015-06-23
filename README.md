# git-tags-rs
Implementation of https://github.com/bmcorser/git-tags-py in Rust

## Setup
To define a directory as a package in your repo, add a `.package` file.

## Lookup logic
get refs for channel
sort by number
checkout the latest
get paths of directories in the working tree with .package file
look at tag contents to get package trees released in latest
iterate back through refs until all packages’ latest trees are known

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
