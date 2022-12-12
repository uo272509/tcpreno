# TCP Reno and Tahoe calculator
A simple calculator for the TCP Reno and Tahoe algorithms.

It outputs a graph and the result in CSV

# Description (`tcpreno --help`)
```bash
Usage: tcpreno [OPTIONS] --cycles <CYCLES> --threshold <THRESHOLD>

Options:
  -c, --cycles <CYCLES>        Number of cycles to calculate
  -t, --threshold <THRESHOLD>  The initial threshold
  -l, --losses <LOSSES>        An array of the cycles on which a loss occurs
  -a, --algorithm <ALGORITHM>  Algorithm to use: 'Tahoe' or 'Reno' [default: Reno]
  -h, --help                   Print help information
  -V, --version                Print version information
```

## Example
> Draw 25 cycles using the algorithm "TCP Reno", where the threshold starts at 8 and
packets are lost in the cycles 10, 14 and 20

```bash
tcpreno --cycles 25 --threshold 8 -l 10 -l 14 -l 20 --algorithm "Reno"
```

# Warning
The algorithm used does not reflect the real-world version, this is merely 
a representation of a simplification we are taught in the TIC course.

