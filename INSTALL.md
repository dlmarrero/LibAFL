# Offline LibAFL Setup Instructions

## Setup

### Optional: Fetching upstream changes

To merge in changes from the main branch of LibAFL:

```sh
git clone https://github.com/dlmarrero/LibAFL.git
cd LibAFL/
git remote add upstream https://github.com/AFLplusplus/LibAFL
git fetch upstream
git merge upstream/main
```

### QEMU Support

Before building, edit `Dockerfile-offline` to support the required
cross-compilation toolchain (if fuzzing with QEMU).

### VSCode Support

This build allows building LibAFL fuzzers in a Docker container. VSCode supports
developing inside a container through use of the [dev containers](https://code.visualstudio.com/docs/devcontainers/containers).
To use this feature, ensure that the dev containers [plugin](https://code.visualstudio.com/docs/devcontainers/containers) is installed in your offline VSCode environment.

Additionally, note the commit ID in your offline VSCode environment by going to
`vscode -> About`. Update the `ARG commit_id` value in `Dockerfile-vscode` to
match. This will ensure the correct version of VSCode Server is installed in
the container, which is required to support dev containers.

## Building the docker images

Included are three Dockerfiles. `Dockerfile` builds the base LibAFL image and is
maintained in the upstream LibAFL repo. `Dockerfile-offline` builds in
offline support. `Dockerfile-vscode` builds offline VSCode support for
developing in the context of a Docker image. It also installs rust-analyzer.

### Build order

```sh
# Build the base LibAFL image
docker build -t libafl:base -f Dockerfile .
# Build the image that allows it to run offline
docker build -t libafl:offline -f Dockerfile-offline .
# (Optional) Add vscode server to the image to support rust-analyzer
docker build -t libafl:vscode -f Dockerfile-vscode .
```

Note that we create each image as `libafl:<tag>`. This enables switching between
images on the offline system, should that become necessary for whatever reason.
You may want to append a version number to the tags if you intend to pull in
upstream updates in the future and retain the ability to roll back.

## Saving/Loading the complete Docker image

With the necessary images built and tagged, the following command will save all
tagged images to a single tarball.

```sh
docker save libafl | gzip > libafl.tar.gz
```

Transfer the file to the offline system and load it into Docker by running:

```sh
docker load -i libafl.tar.gz
```

If you then run `docker images`, the saved images should appear.

## Running VSCode

With the Dev Container plugin installed, open your project folder in code
and enter `Dev Containers`in the command palette to view available options.
Follow the instructions [here](https://code.visualstudio.com/docs/devcontainers/containers)
for additional setup instructions.
