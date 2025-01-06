# CFDSIM: CLI for GMT CFD STARCCM+ sim file

Install with:
```shell
cargo install --git https://github.com/GMTO-Integrated-Modeling/cfdsim.git
```

Get usage with: 
```shell 
cfdsim
```

## Configuration:

The power-on-demand license key is set with the `PODKEY` environment variable

The default path to the "StarCCM+" binary is:
```
/opt/Siemens/17.06.007/STAR-CCM+17.06.007/star/bin/starccm+
```
this can be changed with the `STARCCM` environment variable

The default path to the "StarCCM+" java macros is:
```
/home/ubuntu/Desktop
```
this can be changed with the `STARCCM_MACROS` environment variable
