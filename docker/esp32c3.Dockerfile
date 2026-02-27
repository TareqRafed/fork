FROM espressif/idf:latest
WORKDIR /project
CMD ["idf.py", "build"]
