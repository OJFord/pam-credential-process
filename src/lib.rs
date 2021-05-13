#[macro_use]
extern crate pamsm;

use log::*;
use std::ffi::CString;
use std::process::Command;

use pamsm::Pam;
use pamsm::PamError;
use pamsm::PamFlag;
use pamsm::PamLibExt;
use pamsm::PamServiceModule;

struct PamCredentialProcess;

impl PamServiceModule for PamCredentialProcess {
    fn authenticate(pamh: Pam, flag: PamFlag, args: Vec<String>) -> PamError {
        stderrlog::new()
            .quiet(matches!(flag, PamFlag::PAM_SILENT))
            .verbosity(usize::MAX)
            .init()
            .unwrap();

        let cred_proc = Command::new(&args[0]).args(&args[1..]).output();
        if cred_proc.is_err() {
            error!("Credential process could not be executed");
            return PamError::AUTHINFO_UNAVAIL;
        }

        info!("Credential process executed");
        let cred = cred_proc.unwrap();
        if !cred.status.success() {
            error!("{}", String::from_utf8_lossy(&cred.stderr));
            error!("Credential process exited with non-zero status");
            return PamError::AUTH_ERR;
        }

        info!("Credential process succeeded");
        match CString::new(cred.stdout) {
            Ok(tok) => match pamh.set_authtok(&tok) {
                Ok(_) => PamError::SUCCESS,
                Err(e) => {
                    eprint!("{}", e);
                    PamError::AUTH_ERR
                }
            },
            Err(_) => {
                error!("Token not a valid C string, could not be set");
                PamError::AUTH_ERR
            }
        }
    }
}

pam_module!(PamCredentialProcess);
