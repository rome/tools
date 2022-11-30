FROM centos:centos7
SHELL ["/bin/bash", "--login", "-c"]
RUN yum update -y
RUN yum install gcc gcc-c++ make openssl-devel git -y
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash
RUN nvm install 14
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo install cargo-audit
RUN cargo install cargo-nextest
RUN cargo install just
CMD ["/bin/bash", "--login"]