#!/usr/bin/env bash

# This script ensures that the correct CI variables are set for the build.

if [ -z "$ORG_GRADLE_PROJECT_artifactory_contextUrl" ] ; then
    echo "WARN: ORG_GRADLE_PROJECT_artifactory_contextUrl ci variable not set.  Defaulting to ${ARTIFACT_REPO_URL}."
    export ORG_GRADLE_PROJECT_artifactory_contextUrl=${ARTIFACT_REPO_URL}
fi
if [ -z "$ORG_GRADLE_PROJECT_artifactory_user" ] ; then
    echo "WARN: ORG_GRADLE_PROJECT_artifactory_user ci variable not set.  Defaulting to ${ARTIFACT_REPO_USER}."
    export ORG_GRADLE_PROJECT_artifactory_user=${ARTIFACT_REPO_USER}
fi
if [ -z "$ORG_GRADLE_PROJECT_artifactory_password" ] ; then
    echo "WARN: ORG_GRADLE_PROJECT_artifactory_password ci variable not set.  Using default."
    export ORG_GRADLE_PROJECT_artifactory_password=${ARTIFACT_REPO_PASS}
fi
