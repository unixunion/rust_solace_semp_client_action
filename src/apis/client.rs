use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  about_api: Box<::apis::AboutApi>,
  bridge_api: Box<::apis::BridgeApi>,
  client_api: Box<::apis::ClientApi>,
  default_api: Box<::apis::DefaultApi>,
  distributed_cache_api: Box<::apis::DistributedCacheApi>,
  mqtt_session_api: Box<::apis::MqttSessionApi>,
  msg_vpn_api: Box<::apis::MsgVpnApi>,
  queue_api: Box<::apis::QueueApi>,
  replay_log_api: Box<::apis::ReplayLogApi>,
  rest_delivery_point_api: Box<::apis::RestDeliveryPointApi>,
  topic_endpoint_api: Box<::apis::TopicEndpointApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      about_api: Box::new(::apis::AboutApiClient::new(rc.clone())),
      bridge_api: Box::new(::apis::BridgeApiClient::new(rc.clone())),
      client_api: Box::new(::apis::ClientApiClient::new(rc.clone())),
      default_api: Box::new(::apis::DefaultApiClient::new(rc.clone())),
      distributed_cache_api: Box::new(::apis::DistributedCacheApiClient::new(rc.clone())),
      mqtt_session_api: Box::new(::apis::MqttSessionApiClient::new(rc.clone())),
      msg_vpn_api: Box::new(::apis::MsgVpnApiClient::new(rc.clone())),
      queue_api: Box::new(::apis::QueueApiClient::new(rc.clone())),
      replay_log_api: Box::new(::apis::ReplayLogApiClient::new(rc.clone())),
      rest_delivery_point_api: Box::new(::apis::RestDeliveryPointApiClient::new(rc.clone())),
      topic_endpoint_api: Box::new(::apis::TopicEndpointApiClient::new(rc.clone())),
    }
  }

  pub fn about_api(&self) -> &::apis::AboutApi{
    self.about_api.as_ref()
  }

  pub fn bridge_api(&self) -> &::apis::BridgeApi{
    self.bridge_api.as_ref()
  }

  pub fn client_api(&self) -> &::apis::ClientApi{
    self.client_api.as_ref()
  }

  pub fn default_api(&self) -> &::apis::DefaultApi{
    self.default_api.as_ref()
  }

  pub fn distributed_cache_api(&self) -> &::apis::DistributedCacheApi{
    self.distributed_cache_api.as_ref()
  }

  pub fn mqtt_session_api(&self) -> &::apis::MqttSessionApi{
    self.mqtt_session_api.as_ref()
  }

  pub fn msg_vpn_api(&self) -> &::apis::MsgVpnApi{
    self.msg_vpn_api.as_ref()
  }

  pub fn queue_api(&self) -> &::apis::QueueApi{
    self.queue_api.as_ref()
  }

  pub fn replay_log_api(&self) -> &::apis::ReplayLogApi{
    self.replay_log_api.as_ref()
  }

  pub fn rest_delivery_point_api(&self) -> &::apis::RestDeliveryPointApi{
    self.rest_delivery_point_api.as_ref()
  }

  pub fn topic_endpoint_api(&self) -> &::apis::TopicEndpointApi{
    self.topic_endpoint_api.as_ref()
  }


}
