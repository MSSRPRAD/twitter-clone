#!/bin/bash

docker cp ./scripts/schema.sql 4c9455057884:/home/schema.sql
docker exec -i 4c9455057884 mysql -u admin -ppassword123 mysql-twitterdb <<<"SOURCE /home/schema.sql;"
