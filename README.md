# Overview

Since Mongo Ops Manager does not support sending alerts to Microsoft Teams, this is a rust web server that receives post messages from Ops Manager and forwards them to the specified Teams webhook.

## Using mongo_alerts_2teams

You can either run either the docker container or a compiled binary. 

Docker image: 
```
docker run -p 8000:8000 verticaleap/mongo_alert_2teams:0.1 --url "https://outlook.office.com/webhook/"
```

Binary Executable:
```
cargo install --git https://github.com/findelabs/mongo_alerts_2teams.git --root /usr/local/bin
```

## Configuration

Command line options:
```
--url 
    Specify url for microsoft teams webhook
--port
    Port to listen on
```
