Install postgresql and create database (credential is in env file)

Then in a root of the project, run following commands in order.

cargo install diesel_cli --no-default-features --features postgres

setx LIB "%LIB%;C:\Program Files\PostgreSQL\15\bin"

add system environment variable  C:\Program Files\PostgreSQL\15\lib\

diesel setup

diesel migration run

cargo run

