# twitter-clone

Trying to make a Twitter clone (with it's basic functionalities) using the TRAM stack (Tailwind - React - ActixWeb - MySQL) [forgive me for the acronym :") ]

## SETUP INSTRUCTIONS (Backend and Database)

The project is not well organised and the different components are still not well integrates. So you will have to do a lot of work manually for setting it up.

### Prerequisites

1. Rust, Cargo must be installed.

### Setup Commands

1. Clone the repository

```
git clone https://github.com/mssrprad/twitter-clone.git;
```

<!-- Start the MySql database through docker image -->

```
cd twitter-clone/twitter-backend;
docker-compose up -d;
```

    Sadly the database is not integrating well with sqlx for some reason.
    Sometimes it works and sometimes it doesn't.
    Therefore it is better to create the schema directly inside the docker container.

    Use docker-desktop to access the mysql terminal with the credentials in the
    .env file. Then copy paste these commands
    (also in /home/pradyumnamalladi/twitter-clone/twitter-database/migrations/20230527102513_init.up.sql)

```
CREATE TABLE IF NOT EXISTS ROLES (
    role_id INT PRIMARY KEY AUTO_INCREMENT,
    role_name VARCHAR(10) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS PROFILES (
    profile_id INT PRIMARY KEY AUTO_INCREMENT,
    phone_no CHAR(10) UNIQUE,
    location VARCHAR(20),
    languages VARCHAR(200),
    about VARCHAR(500)
);

CREATE TABLE IF NOT EXISTS USERS (
    user_id INT PRIMARY KEY AUTO_INCREMENT,
    role_id INT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES ROLES(role_id),
    username VARCHAR(20) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    dob CHAR(10) NOT NULL,
    profile_id INT UNIQUE,
    FOREIGN KEY (profile_id) REFERENCES PROFILES(profile_id)
);

INSERT INTO ROLES VALUES(1, "USER");
INSERT INTO ROLES VALUES(2, "ADMIN");
INSERT INTO ROLES VALUES(3, "MODERATOR");
```

his should be enough for the database (for now).
Let's start the backend server now!

```
cd ../twitter-backend/;
cargo watch -q -c -w src/ -x run;
```

In another terminal, let's test the /register endpoint

```
curl -X POST \
-H "Content-Type: application/json" \
-d '{"role_id": 3, "name": "moderatoruser", "email": "moderator@moderator.com","username": "moderator","password": "moderator123","dob": "13/01/2003"}' \
127.0.0.1:8000/register
curl -X POST \
-H "Content-Type: application/json" \
-d '{"role_id": 2, "name": "adminuser", "email": "admin@admin.com","username": "admin","password": "admin123","dob": "13/01/2003"}' \
127.0.0.1:8000/register
curl -X POST \
-H "Content-Type: application/json" \
-d '{"role_id": 1, "name": "normaluser", "email": "user@user.com","username": "user","password": "user123","dob": "13/01/2003"}' \
127.0.0.1:8000/register
```


TODO:

- Make a OpenAPI Document
- Fix Rust Errors
- Make Route Handler Solid
- Performance Measure when getting one vs many
- Quoted Tweet Profile Picture