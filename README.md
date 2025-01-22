# Heater

Simple utility designed to max out CPU utilization for a specified duration.


```
Usage: heater --duration <DURATION>

Options:
  -d, --duration <DURATION>  the duration, in seconds, to run the heater
  -h, --help                 Print help

```

As implemented, for each CPU core, two threads will be spawned with their affinities set to each respective core. 
The threads will spin on CPU-intensive operations for **at least** the duration specified.

Similar to the `stress` utility, except it explicitly tries to set an affinity to for every core to try and ensure 
every core is utilized.