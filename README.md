# Sparkplug Simulator

This is a simple rust program to simulate [Sparkplug](https://sparkplug.eclipse.org/) messages. Once launched it will send sparkplug-compliant messages to an MQTT broker.

It can be used to test services that decode/process Sparkplug messages.

**This software is provided as-is. Use at your own risk. No guarantees are made that the implementation completely matches the Sparkplug specification.**

## Configuration

The simulator is configured by a YAML file `simulator.yaml`. It has the following structure:

```yaml
broker:
    url: tcp://localhost:1883 # URL of the broker
group: mygroup # Group name to use in the MQTT topic hierarchy 
nodes: # A list of nodes to simulate
    - name: foo # The name of the node, will be used as edge_node_id in the MQTT topic hierarchy 
      devices: # A list of devices for the node
        - name: bar # The name of the device, will be used as device_id in the MQTT topic hierarchy
          type: climatecontrol # The type of device to simulate (see below)
```

A sample yaml is included in this repository.

## Devices

The program simulates a number of different devices that emit metrics. Currently the following are implemented:

* `engine`: A simple spinning engine with a single `rpm` metric that changes value at random interval
* `climatecontrol`: A simple climate device with metrics `climateactive` and `temperature` that keeps the temperature between 25 and 30 degrees celcius by activating and deactivating a cooling device.

More devices TBD.

## Running the simulator

Requirements:

* Docker installed
* An MQTT broker, e.g. [HiveMQ CE](https://github.com/hivemq/hivemq-community-edition)

Steps:

1. Adapt and extend the `simulator.yaml` to your liking
2. Launch HiveMQ: `docker run --name hivemq -d -p 1883:1883 hivemq/hivemq-ce`
3. Launch the simulator: `docker run --link hivemq --rm -it -v $(pwd)/simulator.yaml:/simulator.yaml ghcr.io/swoehrl-mw/sparkplug-simulator:latest`
4. Connect to the MQTT broker and watch Sparkplug messages come in (e.g. with the [HiveMQ client](https://github.com/hivemq/hivemq-mqtt-client): `mqtt subscribe -h localhost -p 1883 -t "spBv1.0/#"`)
5. Stop the simulator by issuing `Ctrl-C` (or equivalent on your OS/terminal)

During startup the simulator will send our `NBIRTH` and `DBIRTH` messages. Afterwards it will continuously send out `DDATA` messages until you stop it. During shutdown it will send out `DDEATH` and `NDEATH` messages.

## Restrictions

The simulator currently has the following restrictions:

* No SSL support for the MQTT connection
* No authentication for the MQTT connection
* No rebirth
* `bdSeq` is always 0
* Only a minimal set of Sparkplug is implemented, no metadata, properties, datasets or templates
