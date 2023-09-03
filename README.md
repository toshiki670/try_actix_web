# DB
## DB install
$ brew install postgresql
$ brew services start postgresql
$ diesel setup

## DB access
$ psql -d try_actix_web

## DB migration
$ diesel migration generate users

Add query to up.sql and down.sql

$ diesel migration run

