#!/bin/bash

docker cp ./scripts/schema.sql 1817c7e9defe:/home/schema.sql
# docker exec -it 1b31be7bd292 sh -c "cd /home/; mysql -u admin -p mysql-twitterdb;";
docker exec -i 1817c7e9defe mysql -u admin -ppassword123 mysql-twitterdb  <<< "SOURCE /home/schema.sql;"
