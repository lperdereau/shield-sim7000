[![build](https://github.com/lperdereau/shield-sim7000/workflows/build/badge.svg)](https://github.com/lperdereau/shield-sim7000/actions)
![](https://img.shields.io/crates/d/shield-sim7000.svg)
[![BSD 3-Clause licensed](https://img.shields.io/crates/l/shield-sim7000.svg)](https://github.com/lperdereau/shield-sim7000/blob/master/LICENSE.md)

# Shield SIM7000

This crate provides clients for the SIM7000 series of shields.

| Features | Available (✅/❌) |
|----------|------------------|
| HTTP     | ❌               |
| MQTT     | ❌               |
| TCP      | ❌               |
| GNSS     | ❌               |
| GSM      | ❌               |

# For development

You can use https://www.celersms.com/at-emulator.htm to Emulate a serial port.
Update shell script and add 2001 to `AT_PORT` variable.


Install `socat` to transpose TCP socket into serial port
```
socat pty,link=/dev/virtualcom0,raw tcp:127.0.0.1:2001
```