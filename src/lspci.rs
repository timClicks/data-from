use std::{io::Read, num::ParseIntError, process::Command};

use serde::Serialize;
use erased_serde::Serialize as Erased;
use smol_str::{SmolStr, ToSmolStr};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct PciCode {
    hex: [SmolStr; 2],
    int: [u32; 2],
}

impl PciCode {
    fn try_from_code(code: &str) -> Result<Self, ParseIntError> {
        let left_hex = &code[..2];
        let right_hex = &code[2..];

        let left = u32::from_str_radix(left_hex, 16)?;
        let right = u32::from_str_radix(right_hex, 16)?;

        Ok(PciCode {
            hex: [left_hex.to_smolstr(), right_hex.to_smolstr()],
            int: [left, right],
        })
    }
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Pci {
    slot: Option<SmolStr>,
    device: Option<SmolStr>,
    device_code: Option<PciCode>,
    sub_device: Option<SmolStr>,
    sub_device_code: Option<PciCode>,
    class: Option<SmolStr>,
    class_code: Option<PciCode>,
    vendor: Option<SmolStr>,
    vendor_code: Option<PciCode>,
    sub_vendor: Option<SmolStr>,
    sub_vendor_code: Option<PciCode>,
    revision: Option<u64>,
    programming_interface: Option<SmolStr>,
}

pub fn lspci(use_stdin: bool) -> impl Iterator<Item = Box<dyn Erased>> {
    let mut interfaces = vec![];

    let mut input = String::with_capacity(0x1000);

    if use_stdin {
        let mut stdin = std::io::stdin().lock();
        let _ = stdin.read_to_string(&mut input);
    } else {
        let cmd = Command::new("lspci")
        .args(["-vmm", "-nn"])
        .output()
        .unwrap();

        input = String::from_utf8(cmd.stdout).expect("huh")
    };

    for block in input.trim().split("\n\n") {
        let mut interface = Pci::default();
        for line in block.trim().split('\n') {
            if let Some((key, val)) = line.split_once(':') {
                let val = val.trim();

                let (val, code) = val.rsplit_once(" [")
                    .map(|(v, code)| { (v, code.trim_end_matches(']'))})
                    .unwrap_or((val, ""));

                match key {
                    "Slot" => interface.slot = Some(val.to_smolstr()),
                    "Class" => {
                        interface.class = Some(val.to_smolstr());

                        if let Ok(pci_code_info) = PciCode::try_from_code(&code) {
                            interface.class_code = Some(pci_code_info);
                        };
                    },
                    "Vendor" => {
                        interface.vendor = Some(val.to_smolstr());

                        if let Ok(pci_code_info) = PciCode::try_from_code(&code) {
                            interface.vendor_code = Some(pci_code_info);
                        };
                    },
                    "SVendor" => {
                        interface.sub_vendor = Some(val.to_smolstr());

                        if let Ok(pci_code_info) = PciCode::try_from_code(&code) {
                            interface.sub_vendor_code = Some(pci_code_info);
                        };
                    },
                    "Device" => {
                        interface.device = Some(val.to_smolstr());
                        if let Ok(pci_code_info) = PciCode::try_from_code(&code) {
                            interface.device_code = Some(pci_code_info);
                        };
                    },
                    "SDevice" => {
                        interface.sub_device = Some(val.to_smolstr());
                        if let Ok(pci_code_info) = PciCode::try_from_code(&code) {
                            interface.sub_device_code = Some(pci_code_info);
                        }
                    },
                    "Rev" => {
                        interface.revision = val.parse().ok()
                    },
                    "ProgIf" => interface.programming_interface = Some(val.into()),
                    unknown => eprintln!("warning: unknown key({unknown}) missing from output")
                }
            }
        }
        interfaces.push(interface);
    }

    let iter = interfaces.into_iter().map(|x| Box::new(x) as Box<_>);
    Box::new(iter)
}
