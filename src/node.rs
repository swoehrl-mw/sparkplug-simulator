use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{Broker as BrokerConfig, Node as NodeConfig};
use crate::device::*;
use crate::mqtt::MqttClient;
use crate::sparkplug;

pub struct Node {
    mqtt: MqttClient,
    seq: sparkplug::Sequence,
    group: String,
    name: String,
    devices: Vec<Box<dyn Device>>,
}

fn topic_device(
    group_name: &String,
    node_name: &String,
    message_type: &str,
    device: &Box<dyn Device>,
) -> String {
    format!(
        "spBv1.0/{}/{}/{}/{}",
        group_name,
        message_type,
        node_name,
        device.name()
    )
}

fn topic(group_name: &String, node_name: &String, message_type: &str) -> String {
    format!("spBv1.0/{}/{}/{}", group_name, message_type, node_name)
}

impl Node {
    pub fn new(broker_config: &BrokerConfig, node_config: &NodeConfig, group: String) -> Node {
        let lwt = crate::mqtt::LWT {
            topic: topic(&group, &node_config.name, "NDEATH"),
            payload: sparkplug::ndeath(0),
        };
        let mqtt =
            crate::mqtt::MqttClient::new(broker_config.url.clone(), node_config.name.clone(), lwt);
        let mut devices = Vec::new();
        for device_config in node_config.devices.iter() {
            devices.push(new_device(device_config));
        }
        Node {
            mqtt,
            seq: sparkplug::Sequence::new(),
            group,
            name: node_config.name.clone(),
            devices,
        }
    }

    pub fn run(&mut self, running: Arc<AtomicBool>) {
        // Send birth messages
        self.mqtt.publish(
            topic(&self.group, &self.name, "NBIRTH"),
            sparkplug::nbirth(0),
        );
        for device in self.devices.iter() {
            self.mqtt.publish(
                topic_device(&self.group, &self.name, "DBIRTH", device.clone()),
                sparkplug::dbirth(self.seq.next(), &device.metrics_config()),
            );
        }

        // Simulate activity
        while running.load(Ordering::Relaxed) {
            for device in self.devices.iter_mut() {
                let values = device.as_mut().tick();
                if !values.is_empty() {
                    self.mqtt.publish(
                        topic_device(&self.group, &self.name, "DDATA", device),
                        sparkplug::ddata(self.seq.next(), values),
                    );
                }
            }
            std::thread::sleep(Duration::from_secs(1));
        }

        // Send death messages
        for device in self.devices.iter() {
            self.mqtt.publish(
                topic_device(&self.group, &self.name, "DDEATH", device),
                sparkplug::ddeath(self.seq.next()),
            );
        }
        self.mqtt.publish(
            topic(&self.group, &self.name, "NDEATH"),
            sparkplug::ndeath(0),
        );
        self.mqtt.disconnect();
    }
}
