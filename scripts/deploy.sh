#!/bin/bash
FILES_TO_MOVE=$((`ls -1d staging/* | wc -l`))
if [ $FILES_TO_MOVE -gt 0 ]; then
  apt-get -y update 
  apt-get -y install git-core
  git config --global user.email 'jim.cooke.aus@gmail.com'
  git config --global user.name 'jimcooke'
  git remote set-url origin https://$GIT_CI_USER:$GIT_CI_PASS@gitlab.com/$CI_PROJECT_PATH.git
  git clone https://$GIT_CI_USER:$GIT_CI_PASS@gitlab.com/$CI_PROJECT_PATH.git &> /dev/null
  cd $GIT_CI_ROOT

  # We need to remove any old patch versions for this <upgrade>.<feature> version
  FEATURE_VERSION=$(cat VERSION | cut -f1-2 -d '.')
  PACKAGE=$(grep -m1 name Cargo.toml | cut -f2 -d '=' | tr -d ' "')
  rm -f downloads/${PACKAGE}_${FEATURE_VERSION}*

  # Move the new binary from staging to /downloads as a release
  mv staging/* downloads/
  rm -f .VERSION_BACKUP
  git add --all
  git commit -m "GitLab Runner Push"
  git tag -a v$(cat VERSION) -m "$(date +'%d/%m/%Y') Copyright ©$(date +'%Y') by Jim Cooke"
  git push https://$GIT_CI_USER:$GIT_CI_PASS@gitlab.com/$CI_PROJECT_PATH.git HEAD:master
else
  echo "Warning: No staging files were actually deployed"
  exit 1
fi
