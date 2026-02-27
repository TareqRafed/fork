FROM rustembedded/os-check:latest
WORKDIR /project
CMD ["cargo", "build", "--release"]
