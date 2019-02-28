#!/usr/bin/env bash

# This script ensures that the correct branch is checked out
# for the versioning plugin to calculate the version (using git-flow)

# Gitlab 8 convention
BRANCH_NAME="${CI_BUILD_REF_NAME}"
if [ -z "${BRANCH_NAME}" ] ; then
    # Gitlab 9+ convention
    BRANCH_NAME="${CI_COMMIT_REF_NAME}"
fi

BRANCH_EXISTS=$(git branch | grep -q "$BRANCH_NAME$" && echo "true" || echo "false")
IS_CURRENTLY_CHECKED_OUT=$(git branch | grep "$BRANCH_NAME$" | grep -q "^*" && echo "true" || echo "false")

if [ "$IS_CURRENTLY_CHECKED_OUT" == "false" ] ; then
    if [ "$BRANCH_EXISTS" == "true" ] ; then
        git branch -D $BRANCH_NAME
    fi
    git checkout -b $BRANCH_NAME
fi
