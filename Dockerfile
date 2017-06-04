from debian:latest

RUN apt-get update && \
       apt-get install -y \
       libpq5 \
       --no-install-recommends

COPY target/release/webservice /usr/local/bin

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/webservice"]
