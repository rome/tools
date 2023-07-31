cd rome-ci-x86-64-centos7
docker build . --tag ghcr.io/rome/rome-ci-x86-64-centos7:latest
docker push ghcr.io/rome/rome-ci-x86-64-centos7:latest
cd ..