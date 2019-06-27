# Rust API client for solace_semp_client_monitor

SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the  action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters \"q1\" and \"q2\" ; with values \"val1\" and \"val2\" respectively /SEMP/v2/action/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/action/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/action/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/action/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/action/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/action/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/action/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/action/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/action/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/action/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is a hidden maximum as to prevent overloading the system. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/action/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.    

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 2.11.00901000201
- Package version: 9.1.0-201
- Build package: io.swagger.codegen.languages.RustClientCodegen
For more information, please visit [http://www.solace.com](http://www.solace.com)

## Installation
Put the package under your project folder and add the following in import:
```
    "./solace_semp_client_monitor"
```

## Documentation for API Endpoints

All URIs are relative to *http://www.solace.com/SEMP/v2/action*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AboutApi* | [**get_about_api**](docs/AboutApi.md#get_about_api) | **Get** /about/api | Get an API Description object.
*AboutApi* | [**get_about_user**](docs/AboutApi.md#get_about_user) | **Get** /about/user | Get a User object.
*BridgeApi* | [**get_msg_vpn_bridges**](docs/BridgeApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Get a list of Bridge objects.
*ClientApi* | [**get_msg_vpn_clients**](docs/ClientApi.md#get_msg_vpn_clients) | **Get** /msgVpns/{msgVpnName}/clients | Get a list of Client objects.
*DefaultApi* | [**do_msg_vpn_bridge_clear_event**](docs/DefaultApi.md#do_msg_vpn_bridge_clear_event) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/clearEvent | Clear an event for the Bridge so it can be generated anew.
*DefaultApi* | [**do_msg_vpn_bridge_clear_stats**](docs/DefaultApi.md#do_msg_vpn_bridge_clear_stats) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/clearStats | Clear the statistics for the Bridge.
*DefaultApi* | [**do_msg_vpn_bridge_disconnect**](docs/DefaultApi.md#do_msg_vpn_bridge_disconnect) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/disconnect | Disconnect the Bridge.
*DefaultApi* | [**do_msg_vpn_clear_msg_spool_stats**](docs/DefaultApi.md#do_msg_vpn_clear_msg_spool_stats) | **Put** /msgVpns/{msgVpnName}/clearMsgSpoolStats | Clear the message spool statistics for the Message VPN.
*DefaultApi* | [**do_msg_vpn_clear_replication_stats**](docs/DefaultApi.md#do_msg_vpn_clear_replication_stats) | **Put** /msgVpns/{msgVpnName}/clearReplicationStats | Clear the replication statistics for the Message VPN.
*DefaultApi* | [**do_msg_vpn_clear_service_stats**](docs/DefaultApi.md#do_msg_vpn_clear_service_stats) | **Put** /msgVpns/{msgVpnName}/clearServiceStats | Clear the service statistics for the Message VPN.
*DefaultApi* | [**do_msg_vpn_clear_stats**](docs/DefaultApi.md#do_msg_vpn_clear_stats) | **Put** /msgVpns/{msgVpnName}/clearStats | Clear the messaging statistics for the Message VPN.
*DefaultApi* | [**do_msg_vpn_client_clear_stats**](docs/DefaultApi.md#do_msg_vpn_client_clear_stats) | **Put** /msgVpns/{msgVpnName}/clients/{clientName}/clearStats | Clear the statistics for the Client.
*DefaultApi* | [**do_msg_vpn_client_disconnect**](docs/DefaultApi.md#do_msg_vpn_client_disconnect) | **Put** /msgVpns/{msgVpnName}/clients/{clientName}/disconnect | Disconnect the Client.
*DefaultApi* | [**do_msg_vpn_client_transacted_session_delete**](docs/DefaultApi.md#do_msg_vpn_client_transacted_session_delete) | **Put** /msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions/{sessionName}/delete | Delete the Transacted Session.
*DefaultApi* | [**do_msg_vpn_distributed_cache_cluster_instance_backup_cached_msgs**](docs/DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_backup_cached_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/backupCachedMsgs | Backup cached messages of the Cache Instance to disk.
*DefaultApi* | [**do_msg_vpn_distributed_cache_cluster_instance_cancel_backup_cached_msgs**](docs/DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_cancel_backup_cached_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/cancelBackupCachedMsgs | Cancel the backup of cached messages from the Cache Instance.
*DefaultApi* | [**do_msg_vpn_distributed_cache_cluster_instance_cancel_restore_cached_msgs**](docs/DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_cancel_restore_cached_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/cancelRestoreCachedMsgs | Cancel the restore of cached messages to the Cache Instance.
*DefaultApi* | [**do_msg_vpn_distributed_cache_cluster_instance_clear_event**](docs/DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_clear_event) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/clearEvent | Clear an event for the Cache Instance so it can be generated anew.
*DefaultApi* | [**do_msg_vpn_distributed_cache_cluster_instance_clear_stats**](docs/DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_clear_stats) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/clearStats | Clear the statistics for the Cache Instance.
*DefaultApi* | [**do_msg_vpn_distributed_cache_cluster_instance_delete_msgs**](docs/DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_delete_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/deleteMsgs | Delete messages covered by the given topic in the Cache Instance.
*DefaultApi* | [**do_msg_vpn_distributed_cache_cluster_instance_restore_cached_msgs**](docs/DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_restore_cached_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/restoreCachedMsgs | Restore cached messages for the Cache Instance from disk.
*DefaultApi* | [**do_msg_vpn_distributed_cache_cluster_instance_start**](docs/DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_start) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/start | Start the Cache Instance.
*DefaultApi* | [**do_msg_vpn_mqtt_session_clear_stats**](docs/DefaultApi.md#do_msg_vpn_mqtt_session_clear_stats) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/clearStats | Clear the statistics for the MQTT Session.
*DefaultApi* | [**do_msg_vpn_queue_cancel_replay**](docs/DefaultApi.md#do_msg_vpn_queue_cancel_replay) | **Put** /msgVpns/{msgVpnName}/queues/{queueName}/cancelReplay | Cancel the replay of messages to the Queue.
*DefaultApi* | [**do_msg_vpn_queue_clear_stats**](docs/DefaultApi.md#do_msg_vpn_queue_clear_stats) | **Put** /msgVpns/{msgVpnName}/queues/{queueName}/clearStats | Clear the statistics for the Queue.
*DefaultApi* | [**do_msg_vpn_queue_msg_delete**](docs/DefaultApi.md#do_msg_vpn_queue_msg_delete) | **Put** /msgVpns/{msgVpnName}/queues/{queueName}/msgs/{msgId}/delete | Delete the Message from the Queue.
*DefaultApi* | [**do_msg_vpn_queue_start_replay**](docs/DefaultApi.md#do_msg_vpn_queue_start_replay) | **Put** /msgVpns/{msgVpnName}/queues/{queueName}/startReplay | Start the replay of messages to the Queue.
*DefaultApi* | [**do_msg_vpn_replay_log_trim_logged_msgs**](docs/DefaultApi.md#do_msg_vpn_replay_log_trim_logged_msgs) | **Put** /msgVpns/{msgVpnName}/replayLogs/{replayLogName}/trimLoggedMsgs | Trim (delete) messages from the Replay Log.
*DefaultApi* | [**do_msg_vpn_rest_delivery_point_rest_consumer_clear_stats**](docs/DefaultApi.md#do_msg_vpn_rest_delivery_point_rest_consumer_clear_stats) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/clearStats | Clear the statistics for the REST Consumer.
*DefaultApi* | [**do_msg_vpn_topic_endpoint_cancel_replay**](docs/DefaultApi.md#do_msg_vpn_topic_endpoint_cancel_replay) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/cancelReplay | Cancel the replay of messages to the Topic Endpoint.
*DefaultApi* | [**do_msg_vpn_topic_endpoint_clear_stats**](docs/DefaultApi.md#do_msg_vpn_topic_endpoint_clear_stats) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/clearStats | Clear the statistics for the Topic Endpoint.
*DefaultApi* | [**do_msg_vpn_topic_endpoint_msg_delete**](docs/DefaultApi.md#do_msg_vpn_topic_endpoint_msg_delete) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs/{msgId}/delete | Delete the Message from the Topic Endpoint.
*DefaultApi* | [**do_msg_vpn_topic_endpoint_start_replay**](docs/DefaultApi.md#do_msg_vpn_topic_endpoint_start_replay) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/startReplay | Start the replay of messages to the Topic Endpoint.
*DefaultApi* | [**get_about_user_msg_vpn**](docs/DefaultApi.md#get_about_user_msg_vpn) | **Get** /about/user/msgVpns/{msgVpnName} | Get a User Message VPN object.
*DefaultApi* | [**get_about_user_msg_vpns**](docs/DefaultApi.md#get_about_user_msg_vpns) | **Get** /about/user/msgVpns | Get a list of User Message VPN objects.
*DefaultApi* | [**get_msg_vpn**](docs/DefaultApi.md#get_msg_vpn) | **Get** /msgVpns/{msgVpnName} | Get a Message VPN object.
*DefaultApi* | [**get_msg_vpn_bridge**](docs/DefaultApi.md#get_msg_vpn_bridge) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Get a Bridge object.
*DefaultApi* | [**get_msg_vpn_client**](docs/DefaultApi.md#get_msg_vpn_client) | **Get** /msgVpns/{msgVpnName}/clients/{clientName} | Get a Client object.
*DefaultApi* | [**get_msg_vpn_client_transacted_session**](docs/DefaultApi.md#get_msg_vpn_client_transacted_session) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions/{sessionName} | Get a Transacted Session object.
*DefaultApi* | [**get_msg_vpn_client_transacted_sessions**](docs/DefaultApi.md#get_msg_vpn_client_transacted_sessions) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions | Get a list of Transacted Session objects.
*DefaultApi* | [**get_msg_vpn_distributed_cache**](docs/DefaultApi.md#get_msg_vpn_distributed_cache) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Get a Distributed Cache object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Get a Cache Cluster object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_instance**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instance) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Get a Cache Instance object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_instances**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instances) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Get a list of Cache Instance objects.
*DefaultApi* | [**get_msg_vpn_distributed_cache_clusters**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Get a list of Cache Cluster objects.
*DefaultApi* | [**get_msg_vpn_mqtt_session**](docs/DefaultApi.md#get_msg_vpn_mqtt_session) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Get an MQTT Session object.
*DefaultApi* | [**get_msg_vpn_queue**](docs/DefaultApi.md#get_msg_vpn_queue) | **Get** /msgVpns/{msgVpnName}/queues/{queueName} | Get a Queue object.
*DefaultApi* | [**get_msg_vpn_queue_msg**](docs/DefaultApi.md#get_msg_vpn_queue_msg) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/msgs/{msgId} | Get a Message object.
*DefaultApi* | [**get_msg_vpn_queue_msgs**](docs/DefaultApi.md#get_msg_vpn_queue_msgs) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/msgs | Get a list of Message objects.
*DefaultApi* | [**get_msg_vpn_replay_log**](docs/DefaultApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Get a Replay Log object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Get a REST Delivery Point object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Get a REST Consumer object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumers**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Get a list of REST Consumer objects.
*DefaultApi* | [**get_msg_vpn_topic_endpoint**](docs/DefaultApi.md#get_msg_vpn_topic_endpoint) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Get a Topic Endpoint object.
*DefaultApi* | [**get_msg_vpn_topic_endpoint_msg**](docs/DefaultApi.md#get_msg_vpn_topic_endpoint_msg) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs/{msgId} | Get a Message object.
*DefaultApi* | [**get_msg_vpn_topic_endpoint_msgs**](docs/DefaultApi.md#get_msg_vpn_topic_endpoint_msgs) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs | Get a list of Message objects.
*DistributedCacheApi* | [**get_msg_vpn_distributed_caches**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_caches) | **Get** /msgVpns/{msgVpnName}/distributedCaches | Get a list of Distributed Cache objects.
*MqttSessionApi* | [**get_msg_vpn_mqtt_sessions**](docs/MqttSessionApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Get a list of MQTT Session objects.
*MsgVpnApi* | [**get_msg_vpn_bridges**](docs/MsgVpnApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Get a list of Bridge objects.
*MsgVpnApi* | [**get_msg_vpn_clients**](docs/MsgVpnApi.md#get_msg_vpn_clients) | **Get** /msgVpns/{msgVpnName}/clients | Get a list of Client objects.
*MsgVpnApi* | [**get_msg_vpn_distributed_caches**](docs/MsgVpnApi.md#get_msg_vpn_distributed_caches) | **Get** /msgVpns/{msgVpnName}/distributedCaches | Get a list of Distributed Cache objects.
*MsgVpnApi* | [**get_msg_vpn_mqtt_sessions**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Get a list of MQTT Session objects.
*MsgVpnApi* | [**get_msg_vpn_queues**](docs/MsgVpnApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Get a list of Queue objects.
*MsgVpnApi* | [**get_msg_vpn_replay_logs**](docs/MsgVpnApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Get a list of Replay Log objects.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_points**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Get a list of REST Delivery Point objects.
*MsgVpnApi* | [**get_msg_vpn_topic_endpoints**](docs/MsgVpnApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Get a list of Topic Endpoint objects.
*MsgVpnApi* | [**get_msg_vpns**](docs/MsgVpnApi.md#get_msg_vpns) | **Get** /msgVpns | Get a list of Message VPN objects.
*QueueApi* | [**get_msg_vpn_queues**](docs/QueueApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Get a list of Queue objects.
*ReplayLogApi* | [**get_msg_vpn_replay_logs**](docs/ReplayLogApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Get a list of Replay Log objects.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_points**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Get a list of REST Delivery Point objects.
*TopicEndpointApi* | [**get_msg_vpn_topic_endpoints**](docs/TopicEndpointApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Get a list of Topic Endpoint objects.


## Documentation For Models

 - [AboutApi](docs/AboutApi.md)
 - [AboutApiLinks](docs/AboutApiLinks.md)
 - [AboutApiResponse](docs/AboutApiResponse.md)
 - [AboutUser](docs/AboutUser.md)
 - [AboutUserLinks](docs/AboutUserLinks.md)
 - [AboutUserMsgVpn](docs/AboutUserMsgVpn.md)
 - [AboutUserMsgVpnLinks](docs/AboutUserMsgVpnLinks.md)
 - [AboutUserMsgVpnResponse](docs/AboutUserMsgVpnResponse.md)
 - [AboutUserMsgVpnsResponse](docs/AboutUserMsgVpnsResponse.md)
 - [AboutUserResponse](docs/AboutUserResponse.md)
 - [MsgVpn](docs/MsgVpn.md)
 - [MsgVpnBridge](docs/MsgVpnBridge.md)
 - [MsgVpnBridgeClearEvent](docs/MsgVpnBridgeClearEvent.md)
 - [MsgVpnBridgeClearStats](docs/MsgVpnBridgeClearStats.md)
 - [MsgVpnBridgeDisconnect](docs/MsgVpnBridgeDisconnect.md)
 - [MsgVpnBridgeLinks](docs/MsgVpnBridgeLinks.md)
 - [MsgVpnBridgeResponse](docs/MsgVpnBridgeResponse.md)
 - [MsgVpnBridgesResponse](docs/MsgVpnBridgesResponse.md)
 - [MsgVpnClearMsgSpoolStats](docs/MsgVpnClearMsgSpoolStats.md)
 - [MsgVpnClearReplicationStats](docs/MsgVpnClearReplicationStats.md)
 - [MsgVpnClearServiceStats](docs/MsgVpnClearServiceStats.md)
 - [MsgVpnClearStats](docs/MsgVpnClearStats.md)
 - [MsgVpnClient](docs/MsgVpnClient.md)
 - [MsgVpnClientClearStats](docs/MsgVpnClientClearStats.md)
 - [MsgVpnClientDisconnect](docs/MsgVpnClientDisconnect.md)
 - [MsgVpnClientLinks](docs/MsgVpnClientLinks.md)
 - [MsgVpnClientResponse](docs/MsgVpnClientResponse.md)
 - [MsgVpnClientTransactedSession](docs/MsgVpnClientTransactedSession.md)
 - [MsgVpnClientTransactedSessionDelete](docs/MsgVpnClientTransactedSessionDelete.md)
 - [MsgVpnClientTransactedSessionLinks](docs/MsgVpnClientTransactedSessionLinks.md)
 - [MsgVpnClientTransactedSessionResponse](docs/MsgVpnClientTransactedSessionResponse.md)
 - [MsgVpnClientTransactedSessionsResponse](docs/MsgVpnClientTransactedSessionsResponse.md)
 - [MsgVpnClientsResponse](docs/MsgVpnClientsResponse.md)
 - [MsgVpnDistributedCache](docs/MsgVpnDistributedCache.md)
 - [MsgVpnDistributedCacheCluster](docs/MsgVpnDistributedCacheCluster.md)
 - [MsgVpnDistributedCacheClusterInstance](docs/MsgVpnDistributedCacheClusterInstance.md)
 - [MsgVpnDistributedCacheClusterInstanceBackupCachedMsgs](docs/MsgVpnDistributedCacheClusterInstanceBackupCachedMsgs.md)
 - [MsgVpnDistributedCacheClusterInstanceCancelBackupCachedMsgs](docs/MsgVpnDistributedCacheClusterInstanceCancelBackupCachedMsgs.md)
 - [MsgVpnDistributedCacheClusterInstanceCancelRestoreCachedMsgs](docs/MsgVpnDistributedCacheClusterInstanceCancelRestoreCachedMsgs.md)
 - [MsgVpnDistributedCacheClusterInstanceClearEvent](docs/MsgVpnDistributedCacheClusterInstanceClearEvent.md)
 - [MsgVpnDistributedCacheClusterInstanceClearStats](docs/MsgVpnDistributedCacheClusterInstanceClearStats.md)
 - [MsgVpnDistributedCacheClusterInstanceDeleteMsgs](docs/MsgVpnDistributedCacheClusterInstanceDeleteMsgs.md)
 - [MsgVpnDistributedCacheClusterInstanceLinks](docs/MsgVpnDistributedCacheClusterInstanceLinks.md)
 - [MsgVpnDistributedCacheClusterInstanceResponse](docs/MsgVpnDistributedCacheClusterInstanceResponse.md)
 - [MsgVpnDistributedCacheClusterInstanceRestoreCachedMsgs](docs/MsgVpnDistributedCacheClusterInstanceRestoreCachedMsgs.md)
 - [MsgVpnDistributedCacheClusterInstanceStart](docs/MsgVpnDistributedCacheClusterInstanceStart.md)
 - [MsgVpnDistributedCacheClusterInstancesResponse](docs/MsgVpnDistributedCacheClusterInstancesResponse.md)
 - [MsgVpnDistributedCacheClusterLinks](docs/MsgVpnDistributedCacheClusterLinks.md)
 - [MsgVpnDistributedCacheClusterResponse](docs/MsgVpnDistributedCacheClusterResponse.md)
 - [MsgVpnDistributedCacheClustersResponse](docs/MsgVpnDistributedCacheClustersResponse.md)
 - [MsgVpnDistributedCacheLinks](docs/MsgVpnDistributedCacheLinks.md)
 - [MsgVpnDistributedCacheResponse](docs/MsgVpnDistributedCacheResponse.md)
 - [MsgVpnDistributedCachesResponse](docs/MsgVpnDistributedCachesResponse.md)
 - [MsgVpnLinks](docs/MsgVpnLinks.md)
 - [MsgVpnMqttSession](docs/MsgVpnMqttSession.md)
 - [MsgVpnMqttSessionClearStats](docs/MsgVpnMqttSessionClearStats.md)
 - [MsgVpnMqttSessionLinks](docs/MsgVpnMqttSessionLinks.md)
 - [MsgVpnMqttSessionResponse](docs/MsgVpnMqttSessionResponse.md)
 - [MsgVpnMqttSessionsResponse](docs/MsgVpnMqttSessionsResponse.md)
 - [MsgVpnQueue](docs/MsgVpnQueue.md)
 - [MsgVpnQueueCancelReplay](docs/MsgVpnQueueCancelReplay.md)
 - [MsgVpnQueueClearStats](docs/MsgVpnQueueClearStats.md)
 - [MsgVpnQueueLinks](docs/MsgVpnQueueLinks.md)
 - [MsgVpnQueueMsg](docs/MsgVpnQueueMsg.md)
 - [MsgVpnQueueMsgDelete](docs/MsgVpnQueueMsgDelete.md)
 - [MsgVpnQueueMsgLinks](docs/MsgVpnQueueMsgLinks.md)
 - [MsgVpnQueueMsgResponse](docs/MsgVpnQueueMsgResponse.md)
 - [MsgVpnQueueMsgsResponse](docs/MsgVpnQueueMsgsResponse.md)
 - [MsgVpnQueueResponse](docs/MsgVpnQueueResponse.md)
 - [MsgVpnQueueStartReplay](docs/MsgVpnQueueStartReplay.md)
 - [MsgVpnQueuesResponse](docs/MsgVpnQueuesResponse.md)
 - [MsgVpnReplayLog](docs/MsgVpnReplayLog.md)
 - [MsgVpnReplayLogLinks](docs/MsgVpnReplayLogLinks.md)
 - [MsgVpnReplayLogResponse](docs/MsgVpnReplayLogResponse.md)
 - [MsgVpnReplayLogTrimLoggedMsgs](docs/MsgVpnReplayLogTrimLoggedMsgs.md)
 - [MsgVpnReplayLogsResponse](docs/MsgVpnReplayLogsResponse.md)
 - [MsgVpnResponse](docs/MsgVpnResponse.md)
 - [MsgVpnRestDeliveryPoint](docs/MsgVpnRestDeliveryPoint.md)
 - [MsgVpnRestDeliveryPointLinks](docs/MsgVpnRestDeliveryPointLinks.md)
 - [MsgVpnRestDeliveryPointResponse](docs/MsgVpnRestDeliveryPointResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumer](docs/MsgVpnRestDeliveryPointRestConsumer.md)
 - [MsgVpnRestDeliveryPointRestConsumerClearStats](docs/MsgVpnRestDeliveryPointRestConsumerClearStats.md)
 - [MsgVpnRestDeliveryPointRestConsumerLinks](docs/MsgVpnRestDeliveryPointRestConsumerLinks.md)
 - [MsgVpnRestDeliveryPointRestConsumerResponse](docs/MsgVpnRestDeliveryPointRestConsumerResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumersResponse](docs/MsgVpnRestDeliveryPointRestConsumersResponse.md)
 - [MsgVpnRestDeliveryPointsResponse](docs/MsgVpnRestDeliveryPointsResponse.md)
 - [MsgVpnTopicEndpoint](docs/MsgVpnTopicEndpoint.md)
 - [MsgVpnTopicEndpointCancelReplay](docs/MsgVpnTopicEndpointCancelReplay.md)
 - [MsgVpnTopicEndpointClearStats](docs/MsgVpnTopicEndpointClearStats.md)
 - [MsgVpnTopicEndpointLinks](docs/MsgVpnTopicEndpointLinks.md)
 - [MsgVpnTopicEndpointMsg](docs/MsgVpnTopicEndpointMsg.md)
 - [MsgVpnTopicEndpointMsgDelete](docs/MsgVpnTopicEndpointMsgDelete.md)
 - [MsgVpnTopicEndpointMsgLinks](docs/MsgVpnTopicEndpointMsgLinks.md)
 - [MsgVpnTopicEndpointMsgResponse](docs/MsgVpnTopicEndpointMsgResponse.md)
 - [MsgVpnTopicEndpointMsgsResponse](docs/MsgVpnTopicEndpointMsgsResponse.md)
 - [MsgVpnTopicEndpointResponse](docs/MsgVpnTopicEndpointResponse.md)
 - [MsgVpnTopicEndpointStartReplay](docs/MsgVpnTopicEndpointStartReplay.md)
 - [MsgVpnTopicEndpointsResponse](docs/MsgVpnTopicEndpointsResponse.md)
 - [MsgVpnsResponse](docs/MsgVpnsResponse.md)
 - [SempError](docs/SempError.md)
 - [SempMeta](docs/SempMeta.md)
 - [SempMetaOnlyResponse](docs/SempMetaOnlyResponse.md)
 - [SempPaging](docs/SempPaging.md)
 - [SempRequest](docs/SempRequest.md)


## Documentation For Authorization

## basicAuth
- **Type**: HTTP basic authentication

Example
```
	auth := context.WithValue(context.TODO(), sw.ContextBasicAuth, sw.BasicAuth{
		UserName: "username",
		Password: "password",
	})
    r, err := client.Service.Operation(auth, args)
```

## Author

support@solace.com

