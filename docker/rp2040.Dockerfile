FROM raspberrypi/pico-sdk:latest
WORKDIR /project
CMD ["cmake", "-B", "build", ".", "&&", "make", "-C", "build"]
