# Overview

Since Mongo Atlas and Ops Manager do not currently support sending alerts to directly Microsoft Teams, this is a rust web server that receives post messages from Atlas or Ops Manager and forwards them to a specified Teams webhook.

## Using mongo_alerts_2teams

You can either run either the docker container or a compiled binary. 

Docker image: 
```
docker run -p 8000:8000 verticaleap/mongo_alert_2teams:0.1 --url "https://outlook.office.com/webhook/"
```

Binary Executable:
```
cargo install --git https://github.com/findelabs/mongo_alerts_2teams.git
```

The executable will be listening on the following paths:
```
/echo:
    This will return the json back to the user, good for debugging
/stdout:
    This will save the posted body to stdout within the container
/alert:
    This will receive posted alerts and transform them before sending them to the Microsoft Teams webhook
/testalert:
    This will receive posted alerts, and return the transformed card back to the client
```

## Configuration

Command line options:
```
--url 
    Specify url for microsoft teams webhook
--port
    Port to listen on
```
