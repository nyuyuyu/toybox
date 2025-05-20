#!/usr/bin/env sh

#
# Argument:
# $1 ... Branch name
#

toybox_usage() {
	echo "Usage: $(basename $0) [branch-name]"
}

if [ -z "$1" ]; then
	toybox_usage
	exit 1
fi

set -euo pipefail

TOYBOX_DEFAULT_BRANCH_NAME=`git symbolic-ref --short HEAD`
TOYBOX_DEFAULT_BRANCH_ROOT_COMMIT=`git rev-list --max-parents=0 "$TOYBOX_DEFAULT_BRANCH_NAME"`
TOYBOX_NEW_BRANCH_NAME="$1"

git checkout --orphan "$TOYBOX_NEW_BRANCH_NAME" "$TOYBOX_DEFAULT_BRANCH_ROOT_COMMIT"
git commit --allow-empty -m 'Initial commit'
