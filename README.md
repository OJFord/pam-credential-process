# pam_credential_process.so

Inspired somewhat by the `credential_process` option for AWS CLI, this is a PAM service module allowing you to authenticate via an arbitrary executable, setting the PAM auth token to its output.

## Installation

Install it at `/usr/lib/security/pam_credential_process.so`, or the equivalent location on your system.

## Usage

```
# In your PAM config, e.g. /etc/pam.d/system-login
auth  optional  pam_credential_process.so  /my/secret/acquisition/program
```
