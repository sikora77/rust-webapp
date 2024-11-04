# Intro
This is a web app made by Jakub Sikorski as a project for the JetBrains internship program


The app is built using rocket as the backend framework, diesel as the ORM and PostgreSQL as the database. The database setup is part of the dockerfile and therefore no setup other than running docker is necessary.
# How to run
```shell linenums="$"
git clone https://github.com/sikora77/rust-webapp.git
cd rust-webapp
docker build -t rust-webapp .
docker run -p 8000:8000 rust-webapp:latest
```
The app will be available at http://0.0.0.0:8000/home