import { invoke } from "@tauri-apps/api";

export enum LogLevel {
    Info = "Info",
    Error = "Error",
    Warn = "Warn",
    Trace = "Trace",
    Debug = "Debug",
    Fatal = "Fatal"
};



export async function invokeWriteLog(level: LogLevel, msg: string) {
    await invoke('cmd_log', {
        level: level,
        message: msg,
    });
}