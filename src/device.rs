use crate::config::{Device as DeviceConfig, DeviceType};
use crate::protobuf::DataType;
use crate::sparkplug::{MetricConfig, Value};
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use rand::Rng;

lazy_static! {
    static ref TICKS: Uniform<u32> = Uniform::from(1..10);
    static ref RPMS: Uniform<u32> = Uniform::from(100..500);
    static ref METRIC_RPM: MetricConfig = MetricConfig {
        name: "rpm".to_string(),
        datatype: DataType::Int32,
        initial_value: Value::Int32(0)
    };
    static ref METRIC_TEMPERATURE: MetricConfig = MetricConfig {
        name: "temperature".to_string(),
        datatype: DataType::Float,
        initial_value: Value::Float32(20.0)
    };
    static ref METRIC_CLIMATEACTIVE: MetricConfig = MetricConfig {
        name: "climateactive".to_string(),
        datatype: DataType::Boolean,
        initial_value: Value::Bool(false)
    };
}

pub trait Device {
    fn metrics_config(&self) -> Vec<MetricConfig>;
    fn tick(&mut self) -> Vec<(MetricConfig, Value)>;
    fn name(&self) -> String;
}

pub struct Engine {
    name: String,
    last_value: u32,
    next_change: u32,
    rng: ThreadRng,
}

impl Engine {
    pub fn new(config: &DeviceConfig) -> Engine {
        Engine {
            name: config.name.clone(),
            last_value: 0,
            next_change: 0,
            rng: rand::thread_rng(),
        }
    }
}

impl Device for Engine {
    fn name(&self) -> String {
        return self.name.clone();
    }
    fn metrics_config(&self) -> Vec<MetricConfig> {
        let mut metrics = Vec::new();
        metrics.push(METRIC_RPM.clone());
        metrics
    }

    fn tick(&mut self) -> Vec<(MetricConfig, Value)> {
        let mut metrics = Vec::new();
        if self.next_change == 0 {
            let next_value = RPMS.sample(&mut self.rng);
            if next_value != self.last_value {
                metrics.push((METRIC_RPM.clone(), Value::Int32(next_value)));
            }
            self.next_change = TICKS.sample(&mut self.rng);
        } else {
            self.next_change -= 1;
        }
        metrics
    }
}

pub struct ClimateControl {
    name: String,
    active: bool,
    last_temperature: f32,
    rng: ThreadRng,
}

impl ClimateControl {
    pub fn new(config: &DeviceConfig) -> ClimateControl {
        ClimateControl {
            name: config.name.clone(),
            active: false,
            last_temperature: 20.0,
            rng: rand::thread_rng(),
        }
    }
}

impl Device for ClimateControl {
    fn name(&self) -> String {
        return self.name.clone();
    }
    fn metrics_config(&self) -> Vec<MetricConfig> {
        let mut metrics = Vec::new();
        metrics.push(METRIC_CLIMATEACTIVE.clone());
        metrics.push(METRIC_TEMPERATURE.clone());
        metrics
    }

    fn tick(&mut self) -> Vec<(MetricConfig, Value)> {
        let mut metrics = Vec::new();

        let change = self.rng.gen_range(0.1..1.5) as f32;
        let direction: f32 = if self.active { -1.0 } else { 1.0 };
        self.last_temperature = self.last_temperature + direction * change;
        metrics.push((
            METRIC_TEMPERATURE.clone(),
            Value::Float32(self.last_temperature),
        ));

        let last_state = self.active;

        self.active = if self.active {
            self.last_temperature > 25.0
        } else {
            self.last_temperature > 30.0
        };
        if last_state != self.active {
            metrics.push((METRIC_CLIMATEACTIVE.clone(), Value::Bool(self.active)));
        }
        metrics
    }
}

pub fn new_device(config: &DeviceConfig) -> Box<dyn Device> {
    match config.r#type {
        DeviceType::engine => Box::new(Engine::new(config)),
        DeviceType::climatecontrol => Box::new(ClimateControl::new(config)),
    }
}
