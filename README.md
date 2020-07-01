# vmulti-client
[![crates.io](https://img.shields.io/crates/v/vmulti-client.svg)](https://crates.io/crates/vmulti-client)

Crate for interacting with virtual input driver. Works only on Windows.

## Example

You can find an example API in the examples folder.

## Driver install guide

Before installing the driver, you must disable driver signature verification.
 
1. Download from releases `vmulti_driver.zip` and extract it.
2. Open cmd with admin privileges and move to extracted folder location.
3. Run command `devcon.exe install vmulti.inf djpnewton\vmulti`
