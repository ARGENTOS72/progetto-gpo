# Client

### Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder. 
If you chose to develop with the router feature, you will also have a `views` folder.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform desktop
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

# Docker
In order to setup correctly:

1.  Download [Docker Desktop](https://www.docker.com/products/docker-desktop/) and follow the installer instruction to set it up

2. Go to the `docker` folder and:

- in order to **build** it:
```bash
docker compose --project-name playground up --build
```

- in order to **run** it:
```bash
docker compose --project-name playground start
``` 

- in order to **stop** it:
```bash
docker compose --project-name playground stop
``` 

- in order to **drop** it:
```bash
docker compose --project-name playground down
``` 

# Server

**⚠ Important**
The server requires the [Docker](#docker) to be running

In order to run the playgound server locally, enter the server folder and run this command:
```bash
cargo run
```

Otherwise, if you want an optimized version, build it
```bash
cargo build --release
```
and then run it
```bash
cargo run --release
```

# Technical documentation

https://docs.google.com/document/d/1glRH9g9qlMAR5fdO2wPinhbnBpciO1pjF_jIt1dq_BU/edit?usp=sharing

Svago: https://crazycattle3d.io/
