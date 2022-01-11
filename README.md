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