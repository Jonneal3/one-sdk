#!/bin/sh

VERSION_FILE=$1
RELEASE_LEVEL=$2
RELEASE_PREID=$3

CURRENT_VERSION=$(cat $VERSION_FILE)
RELEASE_VERSION=$(npx semver --silent $CURRENT_VERSION --increment $RELEASE_LEVEL --preid $RELEASE_PREID)
printf "$RELEASE_VERSION" >$VERSION_FILE

echo "RELEASE_VERSION=$RELEASE_VERSION"
