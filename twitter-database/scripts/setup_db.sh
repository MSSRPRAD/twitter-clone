#!/bin/bash

docker cp ./scripts/schema.sql 1b31be7bd292:/home/schema.sql
docker exec -it 1b31be7bd292 sh -c "cd /home/; mysql -u admin -p mysql-twitterdb;";
# docker exec -i 1b31be7bd292 mysql -u admin -p password123 mysql-twitterdb  <<< "SOURCE /home/schema.sql;"
