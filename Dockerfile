FROM rust:latest
LABEL authors="sikora"

WORKDIR /app
COPY . .
ENV PGDATA="/usr/local/pgsql/data"
RUN apt update &&\
apt install postgresql systemctl -y &&\
systemctl restart postgresql.service &&\
systemctl enable postgresql.service &&\
mkdir /usr/local/pgsql &&\
mkdir /usr/local/pgsql_logs &&\
chown postgres /usr/local/pgsql &&\
chown postgres /usr/local/pgsql_logs &&\
su - postgres -c "/usr/lib/postgresql/15/bin/initdb -D /usr/local/pgsql/data" &&\
su - postgres  -c "/usr/lib/postgresql/15/bin/pg_ctl -D /usr/local/pgsql/data -l /usr/local/pgsql_logs/logfile start" &&\
su - postgres  -c "/usr/lib/postgresql/15/bin/createdb blogposts" &&\
rustup override set nightly &&\
cargo install --path  . &&\
cargo install diesel_cli --no-default-features --features postgres &&\
diesel migration run --database-url "postgres://postgres@localhost/blogposts" &&\
    mkdir /app/images &&\
    mkdir /app/avatars &&\
chmod +x ./start_all.sh
CMD ./start_all.sh
EXPOSE 8000
