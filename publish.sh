#!/bin/sh

TARGET_BRANCH="gh-pages"
PUBLIC_DIR=".book"

if [[ $(git status -s) ]]
then
    echo "The working directory is dirty. Please commit any pending changes."
    exit 1;
fi

echo "Deleting old publication"
rm -rf "${PUBLIC_DIR}"
mkdir "${PUBLIC_DIR}"
git worktree prune
rm -rf ".git/worktrees/${PUBLIC_DIR}/"

echo "Checking out branch ${TARGET_BRANCH} into ${PUBLIC_DIR}"
git worktree add -B "${TARGET_BRANCH}" "${PUBLIC_DIR}" "origin/${TARGET_BRANCH}"

echo "Removing existing files"
rm -rf "${PUBLIC_DIR}/*"

echo "Generating site"
mdbook build -d _tmp_book
cp -rp _tmp_book/* "${PUBLIC_DIR}/"
rm -rf _tmp_book

echo "Updating branch ${TARGET_BRANCH}"
cd "${PUBLIC_DIR}" && git add --all && \
    git commit -m "Publishing via publish.sh at $(date)" && \
    git push $1
