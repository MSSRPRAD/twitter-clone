#!/bin/bash

docker cp ./scripts/schema.sql b707332d8150:/home/schema.sql
docker exec -i b707332d8150 mysql -u admin -ppassword123 mysql-twitterdb <<<"SOURCE /home/schema.sql;"
