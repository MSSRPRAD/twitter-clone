#!/bin/bash

DB_HOST="localhost"
DB_USER="admin"
DB_PASS="password123"
DB_NAME="mysql-twitterdb"

SQL_FILE="../migrations/20230527102513_init.up.sql"

mysql -h "$DB_HOST" -u "$DB_USER" -p "$DB_PASS" "$DBNAME" < "$SQL_FILE"