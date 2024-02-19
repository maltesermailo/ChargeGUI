// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::AppHandle;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

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
fn load_file(app: AppHandle, state: tauri::State<ChargeState>, file: String) {
    println!("{}", file);

    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(e) => {
            panic!("{}", format!("Can't load configuration file! Error: {}", e));
        }
    };

    //Parse config file
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading config");

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

    app.emit_all("navigate", "/syscalls").unwrap();
}

#[tauri::command]
fn get_syscall_list(state: tauri::State<ChargeState>) -> SyscallsList {
    let mut data = state.syscalls.lock().unwrap();

    println!("Syscalls called");

    return SyscallsList {syscalls: data.to_vec()};
}

fn main() {
    tauri::Builder::default()
        .manage(ChargeState { syscalls: Mutex::new(vec![]) })
        .invoke_handler(tauri::generate_handler![ready, load_file, get_syscall_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
