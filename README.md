# Overview

Since Mongo Atlas and Ops Manager do not currently support sending alerts to directly Microsoft Teams, this is a rust web server that receives post messages from Atlas or Ops Manager and forwards them to a specified Teams webhook.

## Using mongo_alerts_2teams

You can either run either the docker container or a compiled binary. 

Docker image: 
```
docker run -p 8000:8000 verticaleap/mongo_alerts_2teams:latest --url "https://outlook.office.com/webhook/..."
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
/alert?channel=$CHANNEL:
    This will receive posted alerts and transform them before sending them to the Microsoft Teams webhook
/testalert:
    This will receive posted alerts, and return the transformed card back to the client
```

## Configuration

Command line options:
```
--config
    Specify url for microsoft teams webhook
--port
    Port to listen on
```

You will need to specify a configuration file that lists each channel and corresponding Teams webhook. Then specify which channel to alert by passing the channel to /alert. An example is shown here:
```
endpointone: "https://outlook.office.com/webhook/"
endpointtwo: "https://outlook.office.com/webhook/"
```

To send an alert to endpointone, configure Ops Manager to send the alert to http://localhost:8000/alert?channel=endpointone

## Testing

You can use the following alert for testing your deployment:
```
curl -s http://localhost:8000/alert?channel=endpointone -d "{
  \"alertConfigId\":\"5d2f6c93aa9b4c5735a37475\",
  \"clusterId\":\"5e1cc8d07a924c28f0847da8\",
  \"clusterName\":\"replicaset_name\",
  \"created\":\"2020-07-23T18:24:06Z\",
  \"eventTypeName\":\"PRIMARY_ELECTED\",
  \"groupId\":\"5e11d830a59b4c5bd7dd011a\",
  \"hostId\":\"8561bbbdc6d06fed0f983a344aa8366a\",
  \"hostnameAndPort\":\"localhost:27017\",
  \"id\":\"5f19a5c654964862806e9715\",
  \"links\":[
    {
      \"href\":\"http://localhost:8080/api/public/v1.0/groups/5e1ad8b08a9b4c5bd71d011a/alerts/5f1fd5c6549d486d80719715\",
  \"rel\":\"self\"
    }
  ],
  \"replicaSetName\":\"replicaset_name\",
  \"status\":\"TEST ALERT\",
  \"typeName\":\"REPLICA_SET\",
  \"updated\":\"2020-07-23T18:24:06Z\"
}" 

```
