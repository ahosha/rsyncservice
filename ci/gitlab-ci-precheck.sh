#!/usr/bin/env bash

# This script ensures that the correct CI variables are set for the build.

RC=0
if [ -z "$ORG_GRADLE_PROJECT_artifactory_contextUrl" ] ; then
    echo "ERROR: Require ORG_GRADLE_PROJECT_artifactory_contextUrl ci variable set"
    let RC+=1
fi
if [ -z "$ORG_GRADLE_PROJECT_artifactory_password" ] ; then
    echo "ERROR: Require ORG_GRADLE_PROJECT_artifactory_password ci variable set"
    let RC+=1
fi

if [ -z "$ORG_GRADLE_PROJECT_artifactory_user" ] ; then
    echo "ERROR: Require ORG_GRADLE_PROJECT_artifactory_user ci variable set"
    let RC+=1
fi
if [ $RC -ne 0 ] ; then
    exit $RC
fi

