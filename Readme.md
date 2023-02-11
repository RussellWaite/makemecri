# CRI-API client aka makemecri

This is just a proof of concept for a much more ambitious project called Assembly Line over at https://github.com/null-channel/AssemblyLine

At the moment, the code has everything hard-coded and so can only run a busybox image which opens a `netcat` command on port 9009 for 20 seconds, the app also waits 20 seconds before it destroys the pod that the container was created within.

Future work on this will be to get volumes working, then as a mere proof of concept, it will be classed as finished.

The build.rs file uses tonic to build the gRPC client for the CRI-API. The code then has a `Containerd` unix socket hardcoded into the connection however the theory is that should be able to communicate with any CRI compliant service. 

## Volume testing

The latest update hardcodes `/input/` and `/output/` volumes mapping to the host's  `/tmp/input/` and `/tmp/output/` directories with a `val` file in the input which just contains some text and a `something.sh` script which uses the val file to read the input and write it to the `/output/` folder. The script also runs some `ls` commands to show directory contents and also tries to write to the readonly volume `/input/`. 

These are the contents of the `something.sh` script:

```sh /tmp/input/something.sh
#!/bin/sh
cat /input/val >> /output/result
echo "Just copied val contents into result"
ls -alh /
ls -alh /output
# this should fail as it should be readonly
touch /input/shouldfail
```

The results of all these commands (i.e. stdout) will be stored in the log, which is hardcoded to `/tmp/busybox.log` (which will be mapped to the host under `/tmp/tmp/busybox.log`)

There were some interesting failures with sending `>`, `&&` and `>>` via the arguments vector. Might have to dig a bit deeper into that another time as it might just have been a issue when showing the container details using `crictl inspect`.