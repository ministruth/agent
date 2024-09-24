# agent

Agent for `monitor` plugin. You can run this agent on the server to monitor in Skynet.

## Quick start

Assume the Skynet `monitor` plugin is listening on `localhost:4242` and you are running on 64bit linux:

1. Download server public key to `pubkey`.
2. `./agent_linux_x64 list` to view system info.
3. Look for disks and interfaces you want to monitor, suppose the output is like this:
   ```
    Disks:
    [/dev/vda1][ext4][HDD] -> / (...)
    [/dev/vda2][ext4][HDD] -> /data (...)

    Interfaces:
    [eth0][...] TX/RX: ...
   ```
4. Run agent with
   ```
   ./agent_linux_x64 run -d /dev/vda1 -d /dev/vda2 -i eth0 localhost:4242 pubkey
   ```
5. You can also add `-p` to make the agent passive (wait for server to connect).

## Run agent as service
You can use the following `systemd` configuration:

```
[Unit]
Description=Skynet agent
After=network.target

[Service]
Type=simple
ExecStart=/[path]/agent_linux_x64 run -d [disk] -i [net] [host] [pubkey]
Restart=always
User=[user]
WorkingDirectory=/[path]/

[Install]
WantedBy=multi-user.target
```

Note that you must change `[disk]`、`[net]`、`[host]`、`[pubkey]`、`[path]`、`[user]` to fit your system.