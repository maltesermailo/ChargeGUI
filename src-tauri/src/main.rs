// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::AppHandle;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

use crate::container::SeccompRule;

pub mod container;

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyscallDefinition {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Syscall {
    pub syscall: SyscallDefinition,
    pub enabled: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyscallsList {
    pub syscalls: Vec<Syscall>,
}

#[derive(Serialize, Deserialize)]
pub struct SyscallEvent {
    pub pid: u32,
    pub syscall_no: u32,
    pub args: [u64; 6],
}

#[derive(Clone, serde::Serialize)]
struct LogMessage {
  message: String,
}

struct ChargeState {
    pub syscalls: Mutex<Vec<Syscall>>
}

fn loadSyscalls() -> Vec<SyscallDefinition> {
    let str = include_str!("syscalls.json");

    let syscallList: Vec<SyscallDefinition> = serde_json::from_str(str).unwrap();

    return syscallList;
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn ready(app: AppHandle) {
    println!("Test");
    std::thread::spawn(move || {
        app.emit_all("log", LogMessage { message: "Hi".into() }).unwrap();
    });
}

#[tauri::command]
fn load_file(app: AppHandle, state: tauri::State<ChargeState>, file: String) -> Result<(), Error> {
    println!("{}", file);

    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(e) => {
            return Err(Error::Io(e));
        }
    };

    //Parse config file
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let mut data = state.syscalls.lock().unwrap();
    data.clear();

    let syscalls = loadSyscalls();

    for syscall in &syscalls {
        data.push(Syscall {
            syscall: syscall.clone(),
            enabled: false
        });
    }
    
    for line in contents.lines() {
        let syscallEvent: SyscallEvent = serde_json::from_str(line).unwrap();
        let sysno: u32 = syscallEvent.syscall_no;
        
        if syscalls.get(sysno as usize).is_some() {
            let syscall = syscalls.get(sysno as usize).unwrap();

            data[sysno as usize].enabled = true;
        }
    }
    //enable read and write by default
    data[0].enabled = true;
    data[1].enabled = true;

    app.emit_all("navigate", "/syscalls").unwrap();
    Ok(())
}

#[tauri::command]
fn get_syscall_list(state: tauri::State<ChargeState>) -> SyscallsList {
    let mut data = state.syscalls.lock().unwrap();

    println!("Syscalls called");

    return SyscallsList {syscalls: data.to_vec()};
}

#[tauri::command]
fn set_syscall_list(state: tauri::State<ChargeState>, syscallList: SyscallsList) -> Result<(), Error> {
    let mut result = state.syscalls.lock();

    if let Err(e) = result {
        panic!("Couldn't lock mutex: {}", e);
    }

    let mut data = result.unwrap();

    data.clone_from_slice(&syscallList.syscalls);

    Ok(())
}

fn add_rules(state: &ChargeState, seccompRule: &mut SeccompRule) {
    let mut result = state.syscalls.lock();

    if let Err(e) = result {
        panic!("Couldn't lock mutex: {}", e);
    }

    let data = result.unwrap();
    let data_clone = data.clone();
    
    let (enabledRules, disabledRules): (Vec<Syscall>, Vec<Syscall>) = data_clone.into_iter().partition(|x| x.enabled);

    if(enabledRules.len() > 0) {
        let enabled_rules_names = enabledRules.into_iter().map(|rule| rule.syscall.name).collect();

        seccompRule.getSpec().addRuleList("SCMP_ACT_ALLOW".to_string(), enabled_rules_names);
    }

    if(disabledRules.len() > 0) {
        let disabled_rules_names = disabledRules.into_iter().map(|rule| rule.syscall.name).collect();

        seccompRule.getSpec().addRuleList("SCMP_ACT_KILL".to_string(), disabled_rules_names);
    }
}

#[tauri::command]
fn export_file(state: tauri::State<ChargeState>, file: String) -> Result<(), Error> {
    println!("{}", file);

    let mut seccompRule = SeccompRule::create("test", "SCMP_ACT_ALLOW", vec!["SCMP_ARCH_X86_64".to_string(), "SCMP_ARCH_AARCH64".to_string()]);
    
    add_rules(&state, &mut seccompRule);

    let out = serde_yaml::to_string(&seccompRule).unwrap();

    std::fs::write(file, out);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(ChargeState { syscalls: Mutex::new(vec![]) })
        .invoke_handler(tauri::generate_handler![ready, load_file, get_syscall_list, set_syscall_list, export_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
