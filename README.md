# docker-monorepo-action
Deploys multiple Dockerfile based on files changed in a monorepo

## Usage
```yml
# .github/workflows/docker.yml
name: Deploy to Docker
on:
  push:
    branches:
      - master
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v1
    - name: Build and push changes
      uses: harrygogonis/docker-monorepo-action@master
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Example

Consider the following monorepo:
```sh
project
├── dashboard
│   ├── Dockerfile
│   └── index.php
├── monitoring
│   ├── Dockerfile
│   └── main.js
├── scripts
   └── make.sh
```

Changes to `monitoring/**` or `dashboard/**` in a push to master will build and deploy a docker image.
Changes to `scripts/**` will not trigger a build.
