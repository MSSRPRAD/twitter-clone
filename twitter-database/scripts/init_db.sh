#!/bin/bash

# Check if the MySQL container is running
if [[ "$(docker ps | grep mysql)" == "" ]]; then
	echo "---------------mysql container is not running---------------"
	if [[ "$(docker images | grep mysql)" == "" ]] then
		echo "---------------using existing mysql image---------------"
		# Build and run the MySQL container
  		docker-compose up -d mysql
	else
		echo "---------------building from scratch---------------"
		# Run container
		docker run 

	done

  
  
fi

# Wait for the container to initialize
echo "MySql Container has started. Congratulations!"
echo "To setup the database run scripts/setup_db.sh"