# evans - random telemetry fuzzing tool

[![Build Status]()]()

![Ron Evans, Apollo 17 EVA](257771main_as17-152-23393_full.jpg)

Evans is a companion tool to [cernan](https://github.com/postmates/cernan) but
is generally useful for sending randomized data into servers that implement:

  * statsd
  * graphite
  * cernan native
  
protocols. 

# Usage 

```
evans 0.1
Brian L. Troutwine <blt@postmates.com>
fuzz generation for telemetry servers

USAGE:
    evans [FLAGS] [OPTIONS]

FLAGS:
        --graphite    Enable graphite fuzzing
    -h, --help        Prints help information
        --native      Enable native fuzzing
        --statsd      Enable statsd fuzzing
    -v                Sets the level of verbosity
    -V, --version     Prints version information

OPTIONS:
        --graphite_port <GRAPHITE_PORT>    graphite port
        --hertz <HOST>                     the host running cernan
        --host <HOST>                      the host running cernan
        --native_port <NATIVE_PORT>        native port
        --statsd_port <STATSD_PORT>        statsd port
```

# License 

evans is copyright © 2017 Postmates, Inc and released to the public under the
terms of the MIT license.
