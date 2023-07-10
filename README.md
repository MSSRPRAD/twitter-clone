# twitter-clone

Trying to make a Twitter clone (with it's basic functionalities) using the TRAM stack (Tailwind - React - ActixWeb - MySQL) [forgive me for the acronym :") ]

## SETUP INSTRUCTIONS (Backend and Database)

The project is not well organised and the different components are still not well integrates. So you will have to do a lot of work manually for setting it up.

### Prerequisites

1. Rust, Cargo must be installed.
2. npm must be installed.

### Setup Commands

1. Clone the repository

```
git clone https://github.com/mssrprad/twitter-clone.git;
```
2. Start the MySql and Redis database through docker image.

```
cd twitter-clone/twitter-backend;
docker-compose up -d;
```

3. Create the necessary tables.

Sadly the database is not integrating well with sqlx for some reason. I have to fix that. Sometimes it works and sometimes it doesn't. Therefore it is better to create the schema directly inside the docker container. The path for the sql file is: `twitter-clone/twitter-database/scripts/schema.sql`

There is an alternate way to do this also. Find the container id of the MySql container and paste in in: `twitter-clone/twitter-database/scripts/setup_db.sh` then run the bash script which will create the necessary tables.


This should be enough for the database (for now).
Let's start the backend server now!

```
cd ../twitter-backend/;
cargo watch -q -c -w src/ -x run;
```

4. Run the frontend.

``` shell
cd frontend;
npm i && npm run dev;
```

TODO:

- Make a OpenAPI Document
- Implement Rust Compiler Suggestions.
- Quoted Tweet Profile Picture
