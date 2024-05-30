# Derivative calculator

- [Derivative calculator](#derivative-calculator)
- [Usage](#usage)
- [How it works](#how-it-works)
- [License](#license)


A simple command line tool for calculating derivatives.  

This project was born as a low-effort challenge to take the mind off of more complicated stuff like compilers and virtual machines.

# Usage

The command line tool is very basic. To derive a function, call the executable with the function definition in quotes:

```bash
dcalc "2*x + 6^x - 31"
```

To change the variable with respect to which to derive, use the `-d` option:

```bash
dcalc "2*y + 6^y - 31" -d y
```

For more info about using the command line, run with the `--help` flag:

```bash
dcalc --help
```

# How it works

# License

This software and all the contents of this repository are published under the [MIT license](LICENSE).