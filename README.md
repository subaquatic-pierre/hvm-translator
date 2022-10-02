# Hack VM Translator

VM Translator implemented for the Hack platform.

Hack virtual machine (.vm) code to Hack assembly (.asm) translator, implemented in Rust

The program generates Hack assembly from Hack Vm

Implemented in rust

## Usage

1. Clone the repo
2. Ensure `hvm-translator` is executable

```
sudo chmod +x ./hvm-translator
```

3. Run translator

```
./hvm-translator filename or directory
```

## Notes

1. If a single file is supplied as an argument, it file should have `.vm` extension
2. If a directory is supplied as an argument, it should contain files with `.vm` extension
3. The directory should contain `Sys.vm` file to be initialized for the hardware simulator
4. Bootstrap is generated if `Sys.vm` is supplied, bootstrap calls `Sys.init` method
5. Only works on Unix platform
