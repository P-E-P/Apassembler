# Apassembler

Tool to generate APAS isa compliant opcode from it's human readable version.

## Usage

```bash
> apassembler [assembly file path]
```

You may display binary output or hexadecimal output with the following flags.

```bash
> apassembler --binary res/test.asm
```

```bash
> apassembler --hex res/test.asm
```

The complete set of available flags and parameters can be retrieved with the `--help` flag.

```bash
> apassembler --help
```

## Disclaimer

This program is a quick prototype and is not intended for real use. It does not
provides any garantee of stability nor correctness.

## TODO

- [X] Instruction type I parsing
- [X] Instruction type II parsing
- [X] Instruction type III parsing
- [X] Instruction type IV parsing
- [X] Instruction type V parsing
- [X] Instruction type VI parsing
- [X] Immediate values
- [X] Pointer operands
- [ ] Anonymous labels
- [X] Negative immediate values
- [ ] Symbolic address immediate value
- [X] Absolute symbolic address resolver
- [ ] Relative symbolic address resolver
- [ ] Raw data parsing
- [ ] Discard comments
- [ ] Raw binary output