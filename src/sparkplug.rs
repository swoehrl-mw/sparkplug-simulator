use crate::protobuf::payload::{metric, Metric};
use crate::protobuf::{DataType, Payload};
use crate::util::now;

#[derive(Clone, Debug)]
pub struct MetricConfig {
    pub name: String,
    pub datatype: DataType,
    pub initial_value: Value,
}

pub fn nbirth(bd_seq: u64) -> Payload {
    let mut payload = Payload::default();
    payload.seq = Some(0);
    let mut metric = Metric::default();
    metric.name = Some(String::from("bdSeq"));
    metric.datatype = Some(DataType::Int64 as u32);
    metric.value = Some(convert_value(Value::Int64(bd_seq)));
    metric.timestamp = Some(now());
    payload.metrics.push(metric);
    payload
}

pub fn dbirth(seq: u64, metrics: &Vec<MetricConfig>) -> Payload {
    let mut payload = Payload::default();
    payload.seq = Some(seq);
    for metric_config in metrics {
        let mut metric = Metric::default();
        metric.name = Some(metric_config.name.clone());
        metric.datatype = Some(metric_config.datatype as u32);
        metric.timestamp = Some(now());
        metric.value = Some(convert_value(metric_config.initial_value.clone()));
        payload.metrics.push(metric);
    }
    payload
}

pub fn ddata(seq: u64, metrics: Vec<(MetricConfig, Value)>) -> Payload {
    let mut payload = Payload::default();
    payload.seq = Some(seq);
    for (config, value) in metrics {
        let mut metric = Metric::default();
        metric.name = Some(config.name.clone());
        metric.datatype = Some(config.datatype as u32);
        metric.value = Some(convert_value(value));
        metric.timestamp = Some(now());
        payload.metrics.push(metric);
    }
    payload
}

pub fn ndeath(bd_seq: u64) -> Payload {
    let mut payload = Payload::default();
    payload.seq = Some(0);
    let mut metric = Metric::default();
    metric.name = Some(String::from("bdSeq"));
    metric.datatype = Some(DataType::Int64 as u32);
    metric.value = Some(convert_value(Value::Int64(bd_seq)));
    metric.timestamp = Some(now());
    payload.metrics.push(metric);
    payload
}

pub fn ddeath(seq: u64) -> Payload {
    let mut payload = Payload::default();
    payload.seq = Some(seq);
    payload
}

#[derive(Clone, Debug)]
pub enum Value {
    Int32(u32),
    Int64(u64),
    Bool(bool),
    Float32(f32),
}

pub fn convert_value(value: Value) -> metric::Value {
    match value {
        Value::Int32(val) => metric::Value::IntValue(val),
        Value::Int64(val) => metric::Value::LongValue(val),
        Value::Bool(val) => metric::Value::BooleanValue(val),
        Value::Float32(val) => metric::Value::FloatValue(val),
    }
}

pub struct Sequence {
    inner: u64,
}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence { inner: 0 }
    }

    pub fn next(&mut self) -> u64 {
        self.inner += 1;
        self.inner %= 256;
        self.inner
    }
}
