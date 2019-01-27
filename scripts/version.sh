#!/bin/bash

if [ ! -f VERSION ]; then
   echo "ERROR: VERSION file not found"
   echo "To make a new one:  echo '1.0.0' > VERSION"
   exit 1
fi

if [[ "$1" != "upgrade" ]] && [[ "$1" != "feature" ]] && [[ "$1" != "patch" ]]; then
   echo "ERROR: You must supply and argument to denote what is being incremented"
   echo "Usage: version.sh <upgrade|feature|patch>"
   exit 1
fi

# Check if there we are already acting as a release candidate.
if [ -f .VERSION_BACKUP ]; then
  # If we are, do nothing.  Issue a warning but exit the script successfully.
  echo "WARNING: we are already building a release candidate.  Version not incremented.  Either deploy current release or delete .VERSION_BACKUP"
  exit 0
else
  # If we are not acting as a release candidate yet, then make a backup version file to denote that we are
  cp -p VERSION .VERSION_BACKUP
fi

VERSION=$(cat VERSION)

if [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  UPGRADE=$((`cat VERSION | cut -f 1 -d '.'`))
  FEATURE=$((`cat VERSION | cut -f 2 -d '.'`))
  PATCH=$((`cat VERSION | cut -f 3 -d '.'`))
else
  echo "ERROR: VERSION file container invalid format: $VERSION"
  echo "It should be like: <upgrade number>.<feature number>.<patch number>"
  exit 1
fi

if [[ "$1" == "upgrade" ]]; then
  UPGRADE=$((UPGRADE+1))
  FEATURE=$((0))
  PATCH=$((0))
fi

if [[ "$1" == "feature" ]]; then
  FEATURE=$((FEATURE+1))
  PATCH=$((0))
fi

if [[ "$1" == "patch" ]]; then
  PATCH=$((PATCH+1))
fi

echo "${UPGRADE}.${FEATURE}.${PATCH}" > VERSION
echo "VERSION file succesfully upgraded to $1 release ${UPGRADE}.${FEATURE}.${PATCH}"
