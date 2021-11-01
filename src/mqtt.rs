use crate::protobuf::Payload;
use crate::util::now;
use paho_mqtt;
use prost::Message;
use std::time::Duration;

pub struct MqttClient {
    client: paho_mqtt::Client,
}

pub struct LWT {
    pub topic: String,
    pub payload: Payload,
}

impl MqttClient {
    pub fn new(url: String, client_id: String, lwt: LWT) -> MqttClient {
        let client = paho_mqtt::Client::new((url, client_id)).unwrap();
        let will = paho_mqtt::Message::new(lwt.topic, lwt.payload.encode_to_vec(), 1);
        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .will_message(will)
            .finalize();
        client.connect(conn_opts).unwrap();
        MqttClient { client }
    }

    pub fn publish(&mut self, topic: String, mut payload: Payload) {
        payload.timestamp = Some(now());
        let msg = paho_mqtt::Message::new(topic, payload.encode_to_vec(), 1);
        self.client.publish(msg).unwrap();
    }

    pub fn disconnect(&mut self) {
        self.client.disconnect(None).unwrap();
    }
}
