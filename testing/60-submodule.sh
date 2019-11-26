#!/bin/bash

submodule=$(mktemp -d)
repo=$(mktemp -d)
cd $submodule
git init
git commit --allow-empty -m "Initial commit"
cd $repo
git init
git submodule add $submodule submodule
git commit --allow-empty -m "Initial commit"
touch submodule/a
git diff | cat
