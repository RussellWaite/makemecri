# CRI-API client aka makemecri

This is just a proof of concept for a much more ambitious project called Assembly Line over at https://github.com/null-channel/AssemblyLine

At the moment, the code has everything hard-coded and so can only run a busybox image which opens a `netcat` command on port 9009 for 20 seconds, the app also waits 20 seconds before it destroys the pod that the container was created within.

Future work on this will be to get volumes working, then as a mere proof of concept, it will be classed as finished.

The build.rs file uses tonic to build the gRPC client for the CRI-API. The code then has a `Containerd` unix socket hardcoded into the connection however the theory is that should be able to communicate with any CRI compliant service. 
