FROM postgres:16.6-bullseye

# Install pgvector
RUN apt-get update && apt-get install -y postgresql-server-dev-16 build-essential git \
    && git clone https://github.com/pgvector/pgvector.git \
    && cd pgvector && make && make install \
    && apt-get remove -y build-essential git && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/* pgvector

# RUN mkdir -p /docker-entrypoint-initdb.d
# COPY ./initdb-postgis.sh /docker-entrypoint-initdb.d/10_postgis.sh
# COPY ./update-postgis.sh /usr/local/bin
