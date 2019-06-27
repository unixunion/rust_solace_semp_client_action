# \DefaultApi

All URIs are relative to *http://www.solace.com/SEMP/v2/action*

Method | HTTP request | Description
------------- | ------------- | -------------
[**do_msg_vpn_bridge_clear_event**](DefaultApi.md#do_msg_vpn_bridge_clear_event) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/clearEvent | Clear an event for the Bridge so it can be generated anew.
[**do_msg_vpn_bridge_clear_stats**](DefaultApi.md#do_msg_vpn_bridge_clear_stats) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/clearStats | Clear the statistics for the Bridge.
[**do_msg_vpn_bridge_disconnect**](DefaultApi.md#do_msg_vpn_bridge_disconnect) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/disconnect | Disconnect the Bridge.
[**do_msg_vpn_clear_msg_spool_stats**](DefaultApi.md#do_msg_vpn_clear_msg_spool_stats) | **Put** /msgVpns/{msgVpnName}/clearMsgSpoolStats | Clear the message spool statistics for the Message VPN.
[**do_msg_vpn_clear_replication_stats**](DefaultApi.md#do_msg_vpn_clear_replication_stats) | **Put** /msgVpns/{msgVpnName}/clearReplicationStats | Clear the replication statistics for the Message VPN.
[**do_msg_vpn_clear_service_stats**](DefaultApi.md#do_msg_vpn_clear_service_stats) | **Put** /msgVpns/{msgVpnName}/clearServiceStats | Clear the service statistics for the Message VPN.
[**do_msg_vpn_clear_stats**](DefaultApi.md#do_msg_vpn_clear_stats) | **Put** /msgVpns/{msgVpnName}/clearStats | Clear the messaging statistics for the Message VPN.
[**do_msg_vpn_client_clear_stats**](DefaultApi.md#do_msg_vpn_client_clear_stats) | **Put** /msgVpns/{msgVpnName}/clients/{clientName}/clearStats | Clear the statistics for the Client.
[**do_msg_vpn_client_disconnect**](DefaultApi.md#do_msg_vpn_client_disconnect) | **Put** /msgVpns/{msgVpnName}/clients/{clientName}/disconnect | Disconnect the Client.
[**do_msg_vpn_client_transacted_session_delete**](DefaultApi.md#do_msg_vpn_client_transacted_session_delete) | **Put** /msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions/{sessionName}/delete | Delete the Transacted Session.
[**do_msg_vpn_distributed_cache_cluster_instance_backup_cached_msgs**](DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_backup_cached_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/backupCachedMsgs | Backup cached messages of the Cache Instance to disk.
[**do_msg_vpn_distributed_cache_cluster_instance_cancel_backup_cached_msgs**](DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_cancel_backup_cached_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/cancelBackupCachedMsgs | Cancel the backup of cached messages from the Cache Instance.
[**do_msg_vpn_distributed_cache_cluster_instance_cancel_restore_cached_msgs**](DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_cancel_restore_cached_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/cancelRestoreCachedMsgs | Cancel the restore of cached messages to the Cache Instance.
[**do_msg_vpn_distributed_cache_cluster_instance_clear_event**](DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_clear_event) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/clearEvent | Clear an event for the Cache Instance so it can be generated anew.
[**do_msg_vpn_distributed_cache_cluster_instance_clear_stats**](DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_clear_stats) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/clearStats | Clear the statistics for the Cache Instance.
[**do_msg_vpn_distributed_cache_cluster_instance_delete_msgs**](DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_delete_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/deleteMsgs | Delete messages covered by the given topic in the Cache Instance.
[**do_msg_vpn_distributed_cache_cluster_instance_restore_cached_msgs**](DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_restore_cached_msgs) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/restoreCachedMsgs | Restore cached messages for the Cache Instance from disk.
[**do_msg_vpn_distributed_cache_cluster_instance_start**](DefaultApi.md#do_msg_vpn_distributed_cache_cluster_instance_start) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}/start | Start the Cache Instance.
[**do_msg_vpn_mqtt_session_clear_stats**](DefaultApi.md#do_msg_vpn_mqtt_session_clear_stats) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/clearStats | Clear the statistics for the MQTT Session.
[**do_msg_vpn_queue_cancel_replay**](DefaultApi.md#do_msg_vpn_queue_cancel_replay) | **Put** /msgVpns/{msgVpnName}/queues/{queueName}/cancelReplay | Cancel the replay of messages to the Queue.
[**do_msg_vpn_queue_clear_stats**](DefaultApi.md#do_msg_vpn_queue_clear_stats) | **Put** /msgVpns/{msgVpnName}/queues/{queueName}/clearStats | Clear the statistics for the Queue.
[**do_msg_vpn_queue_msg_delete**](DefaultApi.md#do_msg_vpn_queue_msg_delete) | **Put** /msgVpns/{msgVpnName}/queues/{queueName}/msgs/{msgId}/delete | Delete the Message from the Queue.
[**do_msg_vpn_queue_start_replay**](DefaultApi.md#do_msg_vpn_queue_start_replay) | **Put** /msgVpns/{msgVpnName}/queues/{queueName}/startReplay | Start the replay of messages to the Queue.
[**do_msg_vpn_replay_log_trim_logged_msgs**](DefaultApi.md#do_msg_vpn_replay_log_trim_logged_msgs) | **Put** /msgVpns/{msgVpnName}/replayLogs/{replayLogName}/trimLoggedMsgs | Trim (delete) messages from the Replay Log.
[**do_msg_vpn_rest_delivery_point_rest_consumer_clear_stats**](DefaultApi.md#do_msg_vpn_rest_delivery_point_rest_consumer_clear_stats) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/clearStats | Clear the statistics for the REST Consumer.
[**do_msg_vpn_topic_endpoint_cancel_replay**](DefaultApi.md#do_msg_vpn_topic_endpoint_cancel_replay) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/cancelReplay | Cancel the replay of messages to the Topic Endpoint.
[**do_msg_vpn_topic_endpoint_clear_stats**](DefaultApi.md#do_msg_vpn_topic_endpoint_clear_stats) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/clearStats | Clear the statistics for the Topic Endpoint.
[**do_msg_vpn_topic_endpoint_msg_delete**](DefaultApi.md#do_msg_vpn_topic_endpoint_msg_delete) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs/{msgId}/delete | Delete the Message from the Topic Endpoint.
[**do_msg_vpn_topic_endpoint_start_replay**](DefaultApi.md#do_msg_vpn_topic_endpoint_start_replay) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/startReplay | Start the replay of messages to the Topic Endpoint.
[**get_about_user_msg_vpn**](DefaultApi.md#get_about_user_msg_vpn) | **Get** /about/user/msgVpns/{msgVpnName} | Get a User Message VPN object.
[**get_about_user_msg_vpns**](DefaultApi.md#get_about_user_msg_vpns) | **Get** /about/user/msgVpns | Get a list of User Message VPN objects.
[**get_msg_vpn**](DefaultApi.md#get_msg_vpn) | **Get** /msgVpns/{msgVpnName} | Get a Message VPN object.
[**get_msg_vpn_bridge**](DefaultApi.md#get_msg_vpn_bridge) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Get a Bridge object.
[**get_msg_vpn_client**](DefaultApi.md#get_msg_vpn_client) | **Get** /msgVpns/{msgVpnName}/clients/{clientName} | Get a Client object.
[**get_msg_vpn_client_transacted_session**](DefaultApi.md#get_msg_vpn_client_transacted_session) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions/{sessionName} | Get a Transacted Session object.
[**get_msg_vpn_client_transacted_sessions**](DefaultApi.md#get_msg_vpn_client_transacted_sessions) | **Get** /msgVpns/{msgVpnName}/clients/{clientName}/transactedSessions | Get a list of Transacted Session objects.
[**get_msg_vpn_distributed_cache**](DefaultApi.md#get_msg_vpn_distributed_cache) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Get a Distributed Cache object.
[**get_msg_vpn_distributed_cache_cluster**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Get a Cache Cluster object.
[**get_msg_vpn_distributed_cache_cluster_instance**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instance) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Get a Cache Instance object.
[**get_msg_vpn_distributed_cache_cluster_instances**](DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instances) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Get a list of Cache Instance objects.
[**get_msg_vpn_distributed_cache_clusters**](DefaultApi.md#get_msg_vpn_distributed_cache_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Get a list of Cache Cluster objects.
[**get_msg_vpn_mqtt_session**](DefaultApi.md#get_msg_vpn_mqtt_session) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Get an MQTT Session object.
[**get_msg_vpn_queue**](DefaultApi.md#get_msg_vpn_queue) | **Get** /msgVpns/{msgVpnName}/queues/{queueName} | Get a Queue object.
[**get_msg_vpn_queue_msg**](DefaultApi.md#get_msg_vpn_queue_msg) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/msgs/{msgId} | Get a Message object.
[**get_msg_vpn_queue_msgs**](DefaultApi.md#get_msg_vpn_queue_msgs) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/msgs | Get a list of Message objects.
[**get_msg_vpn_replay_log**](DefaultApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Get a Replay Log object.
[**get_msg_vpn_rest_delivery_point**](DefaultApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Get a REST Delivery Point object.
[**get_msg_vpn_rest_delivery_point_rest_consumer**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Get a REST Consumer object.
[**get_msg_vpn_rest_delivery_point_rest_consumers**](DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Get a list of REST Consumer objects.
[**get_msg_vpn_topic_endpoint**](DefaultApi.md#get_msg_vpn_topic_endpoint) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Get a Topic Endpoint object.
[**get_msg_vpn_topic_endpoint_msg**](DefaultApi.md#get_msg_vpn_topic_endpoint_msg) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs/{msgId} | Get a Message object.
[**get_msg_vpn_topic_endpoint_msgs**](DefaultApi.md#get_msg_vpn_topic_endpoint_msgs) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}/msgs | Get a list of Message objects.


# **do_msg_vpn_bridge_clear_event**
> ::models::SempMetaOnlyResponse do_msg_vpn_bridge_clear_event(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, body)
Clear an event for the Bridge so it can be generated anew.

Clear an event for the Bridge so it can be generated anew.   Attribute|Required|Deprecated :---|:---:|:---: eventName|x|    A SEMP client authorized with a minimum access scope/level of \"global/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
  **body** | [**MsgVpnBridgeClearEvent**](MsgVpnBridgeClearEvent.md)| The Clear Event action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_bridge_clear_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_bridge_clear_stats(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, body)
Clear the statistics for the Bridge.

Clear the statistics for the Bridge.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
  **body** | [**MsgVpnBridgeClearStats**](MsgVpnBridgeClearStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_bridge_disconnect**
> ::models::SempMetaOnlyResponse do_msg_vpn_bridge_disconnect(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, body)
Disconnect the Bridge.

Disconnect the Bridge.    A SEMP client authorized with a minimum access scope/level of \"global/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
  **body** | [**MsgVpnBridgeDisconnect**](MsgVpnBridgeDisconnect.md)| The Disconnect action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_clear_msg_spool_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_clear_msg_spool_stats(ctx, msg_vpn_name, body)
Clear the message spool statistics for the Message VPN.

Clear the message spool statistics for the Message VPN.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **body** | [**MsgVpnClearMsgSpoolStats**](MsgVpnClearMsgSpoolStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_clear_replication_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_clear_replication_stats(ctx, msg_vpn_name, body)
Clear the replication statistics for the Message VPN.

Clear the replication statistics for the Message VPN.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **body** | [**MsgVpnClearReplicationStats**](MsgVpnClearReplicationStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_clear_service_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_clear_service_stats(ctx, msg_vpn_name, body)
Clear the service statistics for the Message VPN.

Clear the service statistics for the Message VPN.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **body** | [**MsgVpnClearServiceStats**](MsgVpnClearServiceStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_clear_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_clear_stats(ctx, msg_vpn_name, body)
Clear the messaging statistics for the Message VPN.

Clear the messaging statistics for the Message VPN.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **body** | [**MsgVpnClearStats**](MsgVpnClearStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_client_clear_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_client_clear_stats(ctx, msg_vpn_name, client_name, body)
Clear the statistics for the Client.

Clear the statistics for the Client.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **client_name** | **String**| The clientName of the Client. | 
  **body** | [**MsgVpnClientClearStats**](MsgVpnClientClearStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_client_disconnect**
> ::models::SempMetaOnlyResponse do_msg_vpn_client_disconnect(ctx, msg_vpn_name, client_name, body)
Disconnect the Client.

Disconnect the Client.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **client_name** | **String**| The clientName of the Client. | 
  **body** | [**MsgVpnClientDisconnect**](MsgVpnClientDisconnect.md)| The Disconnect action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_client_transacted_session_delete**
> ::models::SempMetaOnlyResponse do_msg_vpn_client_transacted_session_delete(ctx, msg_vpn_name, client_name, session_name, body)
Delete the Transacted Session.

Delete the Transacted Session.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **client_name** | **String**| The clientName of the Client. | 
  **session_name** | **String**| The sessionName of the Transacted Session. | 
  **body** | [**MsgVpnClientTransactedSessionDelete**](MsgVpnClientTransactedSessionDelete.md)| The Delete action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_distributed_cache_cluster_instance_backup_cached_msgs**
> ::models::SempMetaOnlyResponse do_msg_vpn_distributed_cache_cluster_instance_backup_cached_msgs(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, body)
Backup cached messages of the Cache Instance to disk.

Backup cached messages of the Cache Instance to disk.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **body** | [**MsgVpnDistributedCacheClusterInstanceBackupCachedMsgs**](MsgVpnDistributedCacheClusterInstanceBackupCachedMsgs.md)| The Backup Cached Messages action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_distributed_cache_cluster_instance_cancel_backup_cached_msgs**
> ::models::SempMetaOnlyResponse do_msg_vpn_distributed_cache_cluster_instance_cancel_backup_cached_msgs(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, body)
Cancel the backup of cached messages from the Cache Instance.

Cancel the backup of cached messages from the Cache Instance.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **body** | [**MsgVpnDistributedCacheClusterInstanceCancelBackupCachedMsgs**](MsgVpnDistributedCacheClusterInstanceCancelBackupCachedMsgs.md)| The Cancel Backup Cached Messages action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_distributed_cache_cluster_instance_cancel_restore_cached_msgs**
> ::models::SempMetaOnlyResponse do_msg_vpn_distributed_cache_cluster_instance_cancel_restore_cached_msgs(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, body)
Cancel the restore of cached messages to the Cache Instance.

Cancel the restore of cached messages to the Cache Instance.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **body** | [**MsgVpnDistributedCacheClusterInstanceCancelRestoreCachedMsgs**](MsgVpnDistributedCacheClusterInstanceCancelRestoreCachedMsgs.md)| The Cancel Restore Cached Messages action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_distributed_cache_cluster_instance_clear_event**
> ::models::SempMetaOnlyResponse do_msg_vpn_distributed_cache_cluster_instance_clear_event(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, body)
Clear an event for the Cache Instance so it can be generated anew.

Clear an event for the Cache Instance so it can be generated anew.   Attribute|Required|Deprecated :---|:---:|:---: eventName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **body** | [**MsgVpnDistributedCacheClusterInstanceClearEvent**](MsgVpnDistributedCacheClusterInstanceClearEvent.md)| The Clear Event action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_distributed_cache_cluster_instance_clear_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_distributed_cache_cluster_instance_clear_stats(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, body)
Clear the statistics for the Cache Instance.

Clear the statistics for the Cache Instance.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **body** | [**MsgVpnDistributedCacheClusterInstanceClearStats**](MsgVpnDistributedCacheClusterInstanceClearStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_distributed_cache_cluster_instance_delete_msgs**
> ::models::SempMetaOnlyResponse do_msg_vpn_distributed_cache_cluster_instance_delete_msgs(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, body)
Delete messages covered by the given topic in the Cache Instance.

Delete messages covered by the given topic in the Cache Instance.   Attribute|Required|Deprecated :---|:---:|:---: topic|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **body** | [**MsgVpnDistributedCacheClusterInstanceDeleteMsgs**](MsgVpnDistributedCacheClusterInstanceDeleteMsgs.md)| The Delete Messages action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_distributed_cache_cluster_instance_restore_cached_msgs**
> ::models::SempMetaOnlyResponse do_msg_vpn_distributed_cache_cluster_instance_restore_cached_msgs(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, body)
Restore cached messages for the Cache Instance from disk.

Restore cached messages for the Cache Instance from disk.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **body** | [**MsgVpnDistributedCacheClusterInstanceRestoreCachedMsgs**](MsgVpnDistributedCacheClusterInstanceRestoreCachedMsgs.md)| The Restore Cached Messages action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_distributed_cache_cluster_instance_start**
> ::models::SempMetaOnlyResponse do_msg_vpn_distributed_cache_cluster_instance_start(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, body)
Start the Cache Instance.

Start the Cache Instance.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
  **body** | [**MsgVpnDistributedCacheClusterInstanceStart**](MsgVpnDistributedCacheClusterInstanceStart.md)| The Start Cache Instance action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_mqtt_session_clear_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_mqtt_session_clear_stats(ctx, msg_vpn_name, mqtt_session_client_id, mqtt_session_virtual_router, body)
Clear the statistics for the MQTT Session.

Clear the statistics for the MQTT Session.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
  **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
  **body** | [**MsgVpnMqttSessionClearStats**](MsgVpnMqttSessionClearStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_queue_cancel_replay**
> ::models::SempMetaOnlyResponse do_msg_vpn_queue_cancel_replay(ctx, msg_vpn_name, queue_name, body)
Cancel the replay of messages to the Queue.

Cancel the replay of messages to the Queue.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **queue_name** | **String**| The queueName of the Queue. | 
  **body** | [**MsgVpnQueueCancelReplay**](MsgVpnQueueCancelReplay.md)| The Cancel Replay action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_queue_clear_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_queue_clear_stats(ctx, msg_vpn_name, queue_name, body)
Clear the statistics for the Queue.

Clear the statistics for the Queue.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **queue_name** | **String**| The queueName of the Queue. | 
  **body** | [**MsgVpnQueueClearStats**](MsgVpnQueueClearStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_queue_msg_delete**
> ::models::SempMetaOnlyResponse do_msg_vpn_queue_msg_delete(ctx, msg_vpn_name, queue_name, msg_id, body)
Delete the Message from the Queue.

Delete the Message from the Queue.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **queue_name** | **String**| The queueName of the Queue. | 
  **msg_id** | **String**| The msgId of the Message. | 
  **body** | [**MsgVpnQueueMsgDelete**](MsgVpnQueueMsgDelete.md)| The Delete action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_queue_start_replay**
> ::models::SempMetaOnlyResponse do_msg_vpn_queue_start_replay(ctx, msg_vpn_name, queue_name, body)
Start the replay of messages to the Queue.

Start the replay of messages to the Queue.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **queue_name** | **String**| The queueName of the Queue. | 
  **body** | [**MsgVpnQueueStartReplay**](MsgVpnQueueStartReplay.md)| The Start Replay action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_replay_log_trim_logged_msgs**
> ::models::SempMetaOnlyResponse do_msg_vpn_replay_log_trim_logged_msgs(ctx, msg_vpn_name, replay_log_name, body)
Trim (delete) messages from the Replay Log.

Trim (delete) messages from the Replay Log.   Attribute|Required|Deprecated :---|:---:|:---: olderThanTime|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
  **body** | [**MsgVpnReplayLogTrimLoggedMsgs**](MsgVpnReplayLogTrimLoggedMsgs.md)| The Trim Logged Messages action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_rest_delivery_point_rest_consumer_clear_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_rest_delivery_point_rest_consumer_clear_stats(ctx, msg_vpn_name, rest_delivery_point_name, rest_consumer_name, body)
Clear the statistics for the REST Consumer.

Clear the statistics for the REST Consumer.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
  **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
  **body** | [**MsgVpnRestDeliveryPointRestConsumerClearStats**](MsgVpnRestDeliveryPointRestConsumerClearStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_topic_endpoint_cancel_replay**
> ::models::SempMetaOnlyResponse do_msg_vpn_topic_endpoint_cancel_replay(ctx, msg_vpn_name, topic_endpoint_name, body)
Cancel the replay of messages to the Topic Endpoint.

Cancel the replay of messages to the Topic Endpoint.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
  **body** | [**MsgVpnTopicEndpointCancelReplay**](MsgVpnTopicEndpointCancelReplay.md)| The Cancel Replay action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_topic_endpoint_clear_stats**
> ::models::SempMetaOnlyResponse do_msg_vpn_topic_endpoint_clear_stats(ctx, msg_vpn_name, topic_endpoint_name, body)
Clear the statistics for the Topic Endpoint.

Clear the statistics for the Topic Endpoint.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
  **body** | [**MsgVpnTopicEndpointClearStats**](MsgVpnTopicEndpointClearStats.md)| The Clear Stats action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_topic_endpoint_msg_delete**
> ::models::SempMetaOnlyResponse do_msg_vpn_topic_endpoint_msg_delete(ctx, msg_vpn_name, topic_endpoint_name, msg_id, body)
Delete the Message from the Topic Endpoint.

Delete the Message from the Topic Endpoint.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
  **msg_id** | **String**| The msgId of the Message. | 
  **body** | [**MsgVpnTopicEndpointMsgDelete**](MsgVpnTopicEndpointMsgDelete.md)| The Delete action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **do_msg_vpn_topic_endpoint_start_replay**
> ::models::SempMetaOnlyResponse do_msg_vpn_topic_endpoint_start_replay(ctx, msg_vpn_name, topic_endpoint_name, body)
Start the replay of messages to the Topic Endpoint.

Start the replay of messages to the Topic Endpoint.    A SEMP client authorized with a minimum access scope/level of \"vpn/readwrite\" is required to perform this operation. Available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
  **body** | [**MsgVpnTopicEndpointStartReplay**](MsgVpnTopicEndpointStartReplay.md)| The Start Replay action&#39;s attributes. | 

### Return type

[**::models::SempMetaOnlyResponse**](SempMetaOnlyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_about_user_msg_vpn**
> ::models::AboutUserMsgVpnResponse get_about_user_msg_vpn(ctx, msg_vpn_name, optional)
Get a User Message VPN object.

Get a User Message VPN object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the User Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the User Message VPN. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::AboutUserMsgVpnResponse**](AboutUserMsgVpnResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_about_user_msg_vpns**
> ::models::AboutUserMsgVpnsResponse get_about_user_msg_vpns(ctx, optional)
Get a list of User Message VPN objects.

Get a list of User Message VPN objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"global/none\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::AboutUserMsgVpnsResponse**](AboutUserMsgVpnsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn**
> ::models::MsgVpnResponse get_msg_vpn(ctx, msg_vpn_name, optional)
Get a Message VPN object.

Get a Message VPN object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnResponse**](MsgVpnResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_bridge**
> ::models::MsgVpnBridgeResponse get_msg_vpn_bridge(ctx, msg_vpn_name, bridge_name, bridge_virtual_router, optional)
Get a Bridge object.

Get a Bridge object.   Attribute|Identifying|Deprecated :---|:---:|:---: bridgeName|x| bridgeVirtualRouter|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **bridge_name** | **String**| The bridgeName of the Bridge. | 
  **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **bridge_name** | **String**| The bridgeName of the Bridge. | 
 **bridge_virtual_router** | **String**| The bridgeVirtualRouter of the Bridge. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnBridgeResponse**](MsgVpnBridgeResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client**
> ::models::MsgVpnClientResponse get_msg_vpn_client(ctx, msg_vpn_name, client_name, optional)
Get a Client object.

Get a Client object.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **client_name** | **String**| The clientName of the Client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **client_name** | **String**| The clientName of the Client. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientResponse**](MsgVpnClientResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_transacted_session**
> ::models::MsgVpnClientTransactedSessionResponse get_msg_vpn_client_transacted_session(ctx, msg_vpn_name, client_name, session_name, optional)
Get a Transacted Session object.

Get a Transacted Session object.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x| sessionName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **client_name** | **String**| The clientName of the Client. | 
  **session_name** | **String**| The sessionName of the Transacted Session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **client_name** | **String**| The clientName of the Client. | 
 **session_name** | **String**| The sessionName of the Transacted Session. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientTransactedSessionResponse**](MsgVpnClientTransactedSessionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_client_transacted_sessions**
> ::models::MsgVpnClientTransactedSessionsResponse get_msg_vpn_client_transacted_sessions(ctx, msg_vpn_name, client_name, optional)
Get a list of Transacted Session objects.

Get a list of Transacted Session objects.   Attribute|Identifying|Deprecated :---|:---:|:---: clientName|x| msgVpnName|x| sessionName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **client_name** | **String**| The clientName of the Client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **client_name** | **String**| The clientName of the Client. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnClientTransactedSessionsResponse**](MsgVpnClientTransactedSessionsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache**
> ::models::MsgVpnDistributedCacheResponse get_msg_vpn_distributed_cache(ctx, msg_vpn_name, cache_name, optional)
Get a Distributed Cache object.

Get a Distributed Cache object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheResponse**](MsgVpnDistributedCacheResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster**
> ::models::MsgVpnDistributedCacheClusterResponse get_msg_vpn_distributed_cache_cluster(ctx, msg_vpn_name, cache_name, cluster_name, optional)
Get a Cache Cluster object.

Get a Cache Cluster object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterResponse**](MsgVpnDistributedCacheClusterResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_instance**
> ::models::MsgVpnDistributedCacheClusterInstanceResponse get_msg_vpn_distributed_cache_cluster_instance(ctx, msg_vpn_name, cache_name, cluster_name, instance_name, optional)
Get a Cache Instance object.

Get a Cache Instance object.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| instanceName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
  **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **instance_name** | **String**| The instanceName of the Cache Instance. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterInstanceResponse**](MsgVpnDistributedCacheClusterInstanceResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_cluster_instances**
> ::models::MsgVpnDistributedCacheClusterInstancesResponse get_msg_vpn_distributed_cache_cluster_instances(ctx, msg_vpn_name, cache_name, cluster_name, optional)
Get a list of Cache Instance objects.

Get a list of Cache Instance objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| instanceName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
  **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **cluster_name** | **String**| The clusterName of the Cache Cluster. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClusterInstancesResponse**](MsgVpnDistributedCacheClusterInstancesResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_distributed_cache_clusters**
> ::models::MsgVpnDistributedCacheClustersResponse get_msg_vpn_distributed_cache_clusters(ctx, msg_vpn_name, cache_name, optional)
Get a list of Cache Cluster objects.

Get a list of Cache Cluster objects.   Attribute|Identifying|Deprecated :---|:---:|:---: cacheName|x| clusterName|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **cache_name** | **String**| The cacheName of the Distributed Cache. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnDistributedCacheClustersResponse**](MsgVpnDistributedCacheClustersResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_mqtt_session**
> ::models::MsgVpnMqttSessionResponse get_msg_vpn_mqtt_session(ctx, msg_vpn_name, mqtt_session_client_id, mqtt_session_virtual_router, optional)
Get an MQTT Session object.

Get an MQTT Session object.   Attribute|Identifying|Deprecated :---|:---:|:---: mqttSessionClientId|x| mqttSessionVirtualRouter|x| msgVpnName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
  **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **mqtt_session_client_id** | **String**| The mqttSessionClientId of the MQTT Session. | 
 **mqtt_session_virtual_router** | **String**| The mqttSessionVirtualRouter of the MQTT Session. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnMqttSessionResponse**](MsgVpnMqttSessionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue**
> ::models::MsgVpnQueueResponse get_msg_vpn_queue(ctx, msg_vpn_name, queue_name, optional)
Get a Queue object.

Get a Queue object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **queue_name** | **String**| The queueName of the Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **queue_name** | **String**| The queueName of the Queue. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueResponse**](MsgVpnQueueResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_msg**
> ::models::MsgVpnQueueMsgResponse get_msg_vpn_queue_msg(ctx, msg_vpn_name, queue_name, msg_id, optional)
Get a Message object.

Get a Message object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **queue_name** | **String**| The queueName of the Queue. | 
  **msg_id** | **String**| The msgId of the Message. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **queue_name** | **String**| The queueName of the Queue. | 
 **msg_id** | **String**| The msgId of the Message. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueMsgResponse**](MsgVpnQueueMsgResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_queue_msgs**
> ::models::MsgVpnQueueMsgsResponse get_msg_vpn_queue_msgs(ctx, msg_vpn_name, queue_name, optional)
Get a list of Message objects.

Get a list of Message objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| queueName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **queue_name** | **String**| The queueName of the Queue. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **queue_name** | **String**| The queueName of the Queue. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnQueueMsgsResponse**](MsgVpnQueueMsgsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_replay_log**
> ::models::MsgVpnReplayLogResponse get_msg_vpn_replay_log(ctx, msg_vpn_name, replay_log_name, optional)
Get a Replay Log object.

Get a Replay Log object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| replayLogName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **replay_log_name** | **String**| The replayLogName of the Replay Log. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnReplayLogResponse**](MsgVpnReplayLogResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point**
> ::models::MsgVpnRestDeliveryPointResponse get_msg_vpn_rest_delivery_point(ctx, msg_vpn_name, rest_delivery_point_name, optional)
Get a REST Delivery Point object.

Get a REST Delivery Point object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointResponse**](MsgVpnRestDeliveryPointResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point_rest_consumer**
> ::models::MsgVpnRestDeliveryPointRestConsumerResponse get_msg_vpn_rest_delivery_point_rest_consumer(ctx, msg_vpn_name, rest_delivery_point_name, rest_consumer_name, optional)
Get a REST Consumer object.

Get a REST Consumer object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restConsumerName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
  **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **rest_consumer_name** | **String**| The restConsumerName of the REST Consumer. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointRestConsumerResponse**](MsgVpnRestDeliveryPointRestConsumerResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_rest_delivery_point_rest_consumers**
> ::models::MsgVpnRestDeliveryPointRestConsumersResponse get_msg_vpn_rest_delivery_point_rest_consumers(ctx, msg_vpn_name, rest_delivery_point_name, optional)
Get a list of REST Consumer objects.

Get a list of REST Consumer objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| restConsumerName|x| restDeliveryPointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **rest_delivery_point_name** | **String**| The restDeliveryPointName of the REST Delivery Point. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnRestDeliveryPointRestConsumersResponse**](MsgVpnRestDeliveryPointRestConsumersResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint**
> ::models::MsgVpnTopicEndpointResponse get_msg_vpn_topic_endpoint(ctx, msg_vpn_name, topic_endpoint_name, optional)
Get a Topic Endpoint object.

Get a Topic Endpoint object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointResponse**](MsgVpnTopicEndpointResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_msg**
> ::models::MsgVpnTopicEndpointMsgResponse get_msg_vpn_topic_endpoint_msg(ctx, msg_vpn_name, topic_endpoint_name, msg_id, optional)
Get a Message object.

Get a Message object.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
  **msg_id** | **String**| The msgId of the Message. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
 **msg_id** | **String**| The msgId of the Message. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointMsgResponse**](MsgVpnTopicEndpointMsgResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_msg_vpn_topic_endpoint_msgs**
> ::models::MsgVpnTopicEndpointMsgsResponse get_msg_vpn_topic_endpoint_msgs(ctx, msg_vpn_name, topic_endpoint_name, optional)
Get a list of Message objects.

Get a list of Message objects.   Attribute|Identifying|Deprecated :---|:---:|:---: msgId|x| msgVpnName|x| topicEndpointName|x|    A SEMP client authorized with a minimum access scope/level of \"vpn/readonly\" is required to perform this operation.  This has been available since 2.11.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
  **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **msg_vpn_name** | **String**| The msgVpnName of the Message VPN. | 
 **topic_endpoint_name** | **String**| The topicEndpointName of the Topic Endpoint. | 
 **count** | **i32**| Limit the count of objects in the response. See the documentation for the &#x60;count&#x60; parameter. | [default to 10]
 **cursor** | **String**| The cursor, or position, for the next page of objects. See the documentation for the &#x60;cursor&#x60; parameter. | 
 **_where** | [**Vec&lt;String&gt;**](String.md)| Include in the response only objects where certain conditions are true. See the the documentation for the &#x60;where&#x60; parameter. | 
 **select** | [**Vec&lt;String&gt;**](String.md)| Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. See the documentation for the &#x60;select&#x60; parameter. | 

### Return type

[**::models::MsgVpnTopicEndpointMsgsResponse**](MsgVpnTopicEndpointMsgsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

