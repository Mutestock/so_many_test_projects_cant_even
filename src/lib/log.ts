import { writeTextFile, BaseDirectory } from "@tauri-apps/api/fs";

export enum LogLevel {
    Info = "Info",
    Error = "Error",
    Warn = "Warn",
    Trace = "Trace",
    Debug = "Debug",
    Fatal = "Fatal"
};



export async function writeLog(code: LogLevel, msg: string) {
    await writeTextFile({
        path: 'log.log',
        contents: `Frontend: Code:${code} Message: ${msg}`
    },
        {
            dir: BaseDirectory.AppData
        }
    )
}