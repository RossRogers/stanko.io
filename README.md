## Building

#### Docker
```Bash
MACHINE="$(gcc -dumpmachine)" docker-compose build
```

#### Ubuntu

1. Install pre-requisites:
  
        sudo apt-get install libpq-dev postgresql postgresql-contrib curl
    
2. Install nightly rust:

        curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
    

3. Setup the database with a new user. Change `foo` and `bar` to your liking:

        sudo su - postgres
        createuser foo --pwprompt 
    
    Type in `bar` as the password.
    
    Now give the user superuser permissions
    
        echo alter user foo with superuser \;  | psql
        exit

4. Next, create the schema:
    
        export DATABASE_URL='postgresql://localhost/talk?user=foo&password=bar&ssl=true'
        diesel migration run
    
5. In the same shell, with `DATABASE_URL` set, run with:

        cargo run
        
6. Navigate to [`http://localhost:3000`](http://localhost:3000) to see the package in action.

   
