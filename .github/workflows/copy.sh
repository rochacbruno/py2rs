#!/bin/bash

# create worktree
git worktree add gh-pages

# set git config
git config user.name "Deploy from CI"
git config user.email ""

# navigate to gh-pages
cd gh-pages

# Delete the ref to avoid keeping history.
git update-ref -d refs/heads/gh-pages
rm -rf *

# copy files from home to gh-pages
cp -r ../book/book/* .

# add, commit, and push to gh-pages
git add .
git commit -m "Deploy $GITHUB_SHA to gh-pages"
git push --force origin gh-pages