# Heater

Simple utility designed to max out CPU utilization for a specified duration.


```
Usage: heater [OPTIONS] --duration <DURATION>

Options:
  -d, --duration <DURATION>
          The duration, in seconds, to run the heater. If a negative value is provided, runs forever
  -t, --threads-per-core <THREADS_PER_CORE>
          The number of threads per core to spawn [default: 2]
  -c, --cores <CORES>
          The number of cores to occupy. If unspecified, uses all cores. If the number specified is higher than the number of CPU cores available, all cores will be used.
  -q, --quiet
          Suppress output messages and warnings
  -h, --help
          Print help
```

Notes:

- Obviously, **use caution**, as stressing all cores can render your system unresponsive for the duration specified
- Due to the nature of the program, the duration is only guaranteed to be AT LEAST the duration specified -- actual duration may be slightly longer (though, in testing, this has not been a problem)
- Relies on [`core_affinity`](https://github.com/Elzair/core_affinity_rs) for CPU counts and setting core affinity

