# Hack VM Translator

VM Translator implemented for the Hack platform

The program generates Hack assembly from Hack Vm

Implemented in rust

## Usage

1. Clone the repo
2. Ensure `vm-translator` is executable

```
sudo chmod +x ./vm-translator
```

3. Run translator

```
./vm-translator filename or directory
```

## Notes

1. The file should have `.vm` extension
2. The directory should contain files with `.vm` extension
3. The directory should contain `Sys.vm` file to be initialized for the hardware simulator
4. Bootstrap is generated if `Sys.vm` is supplied, bootstrap calls `Sys.init` method
5. Only works on Unix platform
