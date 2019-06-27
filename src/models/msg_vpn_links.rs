/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the  action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters \"q1\" and \"q2\" ; with values \"val1\" and \"val2\" respectively /SEMP/v2/action/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/action/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/action/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/action/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/action/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/action/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/action/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/action/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/action/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/action/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is a hidden maximum as to prevent overloading the system. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/action/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.    
 *
 * OpenAPI spec version: 2.11.00901000201
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgVpnLinks {
  /// The URI of this MsgVpn's bridges collection.
  #[serde(rename = "bridgesUri", skip_serializing_if="Option::is_none")]
  bridges_uri: Option<String>,
  /// The URI of this MsgVpn's clearMsgSpoolStats action.
  #[serde(rename = "clearMsgSpoolStatsUri", skip_serializing_if="Option::is_none")]
  clear_msg_spool_stats_uri: Option<String>,
  /// The URI of this MsgVpn's clearReplicationStats action.
  #[serde(rename = "clearReplicationStatsUri", skip_serializing_if="Option::is_none")]
  clear_replication_stats_uri: Option<String>,
  /// The URI of this MsgVpn's clearServiceStats action.
  #[serde(rename = "clearServiceStatsUri", skip_serializing_if="Option::is_none")]
  clear_service_stats_uri: Option<String>,
  /// The URI of this MsgVpn's clearStats action.
  #[serde(rename = "clearStatsUri", skip_serializing_if="Option::is_none")]
  clear_stats_uri: Option<String>,
  /// The URI of this MsgVpn's clients collection.
  #[serde(rename = "clientsUri", skip_serializing_if="Option::is_none")]
  clients_uri: Option<String>,
  /// The URI of this MsgVpn's distributedCaches collection.
  #[serde(rename = "distributedCachesUri", skip_serializing_if="Option::is_none")]
  distributed_caches_uri: Option<String>,
  /// The URI of this MsgVpn's mqttSessions collection.
  #[serde(rename = "mqttSessionsUri", skip_serializing_if="Option::is_none")]
  mqtt_sessions_uri: Option<String>,
  /// The URI of this MsgVpn's queues collection.
  #[serde(rename = "queuesUri", skip_serializing_if="Option::is_none")]
  queues_uri: Option<String>,
  /// The URI of this MsgVpn's replayLogs collection.
  #[serde(rename = "replayLogsUri", skip_serializing_if="Option::is_none")]
  replay_logs_uri: Option<String>,
  /// The URI of this MsgVpn's restDeliveryPoints collection.
  #[serde(rename = "restDeliveryPointsUri", skip_serializing_if="Option::is_none")]
  rest_delivery_points_uri: Option<String>,
  /// The URI of this MsgVpn's topicEndpoints collection.
  #[serde(rename = "topicEndpointsUri", skip_serializing_if="Option::is_none")]
  topic_endpoints_uri: Option<String>,
  /// The URI of this MsgVpn object.
  #[serde(rename = "uri", skip_serializing_if="Option::is_none")]
  uri: Option<String>
}

impl MsgVpnLinks {
  pub fn new() -> MsgVpnLinks {
    MsgVpnLinks {
      bridges_uri: None,
      clear_msg_spool_stats_uri: None,
      clear_replication_stats_uri: None,
      clear_service_stats_uri: None,
      clear_stats_uri: None,
      clients_uri: None,
      distributed_caches_uri: None,
      mqtt_sessions_uri: None,
      queues_uri: None,
      replay_logs_uri: None,
      rest_delivery_points_uri: None,
      topic_endpoints_uri: None,
      uri: None
    }
  }

  pub fn set_bridges_uri(&mut self, bridges_uri: String) {
    self.bridges_uri = Some(bridges_uri);
  }

  pub fn with_bridges_uri(mut self, bridges_uri: String) -> MsgVpnLinks {
    self.bridges_uri = Some(bridges_uri);
    self
  }

  pub fn bridges_uri(&self) -> Option<&String> {
    self.bridges_uri.as_ref()
  }

  pub fn reset_bridges_uri(&mut self) {
    self.bridges_uri = None;
  }

  pub fn set_clear_msg_spool_stats_uri(&mut self, clear_msg_spool_stats_uri: String) {
    self.clear_msg_spool_stats_uri = Some(clear_msg_spool_stats_uri);
  }

  pub fn with_clear_msg_spool_stats_uri(mut self, clear_msg_spool_stats_uri: String) -> MsgVpnLinks {
    self.clear_msg_spool_stats_uri = Some(clear_msg_spool_stats_uri);
    self
  }

  pub fn clear_msg_spool_stats_uri(&self) -> Option<&String> {
    self.clear_msg_spool_stats_uri.as_ref()
  }

  pub fn reset_clear_msg_spool_stats_uri(&mut self) {
    self.clear_msg_spool_stats_uri = None;
  }

  pub fn set_clear_replication_stats_uri(&mut self, clear_replication_stats_uri: String) {
    self.clear_replication_stats_uri = Some(clear_replication_stats_uri);
  }

  pub fn with_clear_replication_stats_uri(mut self, clear_replication_stats_uri: String) -> MsgVpnLinks {
    self.clear_replication_stats_uri = Some(clear_replication_stats_uri);
    self
  }

  pub fn clear_replication_stats_uri(&self) -> Option<&String> {
    self.clear_replication_stats_uri.as_ref()
  }

  pub fn reset_clear_replication_stats_uri(&mut self) {
    self.clear_replication_stats_uri = None;
  }

  pub fn set_clear_service_stats_uri(&mut self, clear_service_stats_uri: String) {
    self.clear_service_stats_uri = Some(clear_service_stats_uri);
  }

  pub fn with_clear_service_stats_uri(mut self, clear_service_stats_uri: String) -> MsgVpnLinks {
    self.clear_service_stats_uri = Some(clear_service_stats_uri);
    self
  }

  pub fn clear_service_stats_uri(&self) -> Option<&String> {
    self.clear_service_stats_uri.as_ref()
  }

  pub fn reset_clear_service_stats_uri(&mut self) {
    self.clear_service_stats_uri = None;
  }

  pub fn set_clear_stats_uri(&mut self, clear_stats_uri: String) {
    self.clear_stats_uri = Some(clear_stats_uri);
  }

  pub fn with_clear_stats_uri(mut self, clear_stats_uri: String) -> MsgVpnLinks {
    self.clear_stats_uri = Some(clear_stats_uri);
    self
  }

  pub fn clear_stats_uri(&self) -> Option<&String> {
    self.clear_stats_uri.as_ref()
  }

  pub fn reset_clear_stats_uri(&mut self) {
    self.clear_stats_uri = None;
  }

  pub fn set_clients_uri(&mut self, clients_uri: String) {
    self.clients_uri = Some(clients_uri);
  }

  pub fn with_clients_uri(mut self, clients_uri: String) -> MsgVpnLinks {
    self.clients_uri = Some(clients_uri);
    self
  }

  pub fn clients_uri(&self) -> Option<&String> {
    self.clients_uri.as_ref()
  }

  pub fn reset_clients_uri(&mut self) {
    self.clients_uri = None;
  }

  pub fn set_distributed_caches_uri(&mut self, distributed_caches_uri: String) {
    self.distributed_caches_uri = Some(distributed_caches_uri);
  }

  pub fn with_distributed_caches_uri(mut self, distributed_caches_uri: String) -> MsgVpnLinks {
    self.distributed_caches_uri = Some(distributed_caches_uri);
    self
  }

  pub fn distributed_caches_uri(&self) -> Option<&String> {
    self.distributed_caches_uri.as_ref()
  }

  pub fn reset_distributed_caches_uri(&mut self) {
    self.distributed_caches_uri = None;
  }

  pub fn set_mqtt_sessions_uri(&mut self, mqtt_sessions_uri: String) {
    self.mqtt_sessions_uri = Some(mqtt_sessions_uri);
  }

  pub fn with_mqtt_sessions_uri(mut self, mqtt_sessions_uri: String) -> MsgVpnLinks {
    self.mqtt_sessions_uri = Some(mqtt_sessions_uri);
    self
  }

  pub fn mqtt_sessions_uri(&self) -> Option<&String> {
    self.mqtt_sessions_uri.as_ref()
  }

  pub fn reset_mqtt_sessions_uri(&mut self) {
    self.mqtt_sessions_uri = None;
  }

  pub fn set_queues_uri(&mut self, queues_uri: String) {
    self.queues_uri = Some(queues_uri);
  }

  pub fn with_queues_uri(mut self, queues_uri: String) -> MsgVpnLinks {
    self.queues_uri = Some(queues_uri);
    self
  }

  pub fn queues_uri(&self) -> Option<&String> {
    self.queues_uri.as_ref()
  }

  pub fn reset_queues_uri(&mut self) {
    self.queues_uri = None;
  }

  pub fn set_replay_logs_uri(&mut self, replay_logs_uri: String) {
    self.replay_logs_uri = Some(replay_logs_uri);
  }

  pub fn with_replay_logs_uri(mut self, replay_logs_uri: String) -> MsgVpnLinks {
    self.replay_logs_uri = Some(replay_logs_uri);
    self
  }

  pub fn replay_logs_uri(&self) -> Option<&String> {
    self.replay_logs_uri.as_ref()
  }

  pub fn reset_replay_logs_uri(&mut self) {
    self.replay_logs_uri = None;
  }

  pub fn set_rest_delivery_points_uri(&mut self, rest_delivery_points_uri: String) {
    self.rest_delivery_points_uri = Some(rest_delivery_points_uri);
  }

  pub fn with_rest_delivery_points_uri(mut self, rest_delivery_points_uri: String) -> MsgVpnLinks {
    self.rest_delivery_points_uri = Some(rest_delivery_points_uri);
    self
  }

  pub fn rest_delivery_points_uri(&self) -> Option<&String> {
    self.rest_delivery_points_uri.as_ref()
  }

  pub fn reset_rest_delivery_points_uri(&mut self) {
    self.rest_delivery_points_uri = None;
  }

  pub fn set_topic_endpoints_uri(&mut self, topic_endpoints_uri: String) {
    self.topic_endpoints_uri = Some(topic_endpoints_uri);
  }

  pub fn with_topic_endpoints_uri(mut self, topic_endpoints_uri: String) -> MsgVpnLinks {
    self.topic_endpoints_uri = Some(topic_endpoints_uri);
    self
  }

  pub fn topic_endpoints_uri(&self) -> Option<&String> {
    self.topic_endpoints_uri.as_ref()
  }

  pub fn reset_topic_endpoints_uri(&mut self) {
    self.topic_endpoints_uri = None;
  }

  pub fn set_uri(&mut self, uri: String) {
    self.uri = Some(uri);
  }

  pub fn with_uri(mut self, uri: String) -> MsgVpnLinks {
    self.uri = Some(uri);
    self
  }

  pub fn uri(&self) -> Option<&String> {
    self.uri.as_ref()
  }

  pub fn reset_uri(&mut self) {
    self.uri = None;
  }

}



