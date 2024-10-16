# Lowbat-daemon

This is a short rust program that is meant to be used within Termux on a rooted device.

## Building

Make sure you have `rust` and `termux-api` packages installed, then just `cargo build`.

## Deploying

Place the resulting binary anywhere you wish and place `batmon-config.json` in the same directory. After that, just run the binary. To set it up as a daemon, you can simply invoke the executable within your `.bashrc` file, or set up your own `termux-services` service.

## Configuration

Edit values in `batmon-config.json` to change properties of the program:

* `polling_rate` - time period (in seconds) between battery level poll events;
* `charge_threshold` - battery charge threshold in percents;
* `warning_period` - duration (in seconds) of the low-battery level warning notification, after which the device shuts down;  
