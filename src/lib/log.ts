import { writeTextFile, BaseDirectory, readTextFile } from "@tauri-apps/api/fs";

export enum LogLevel {
    Info = "Info",
    Error = "Error",
    Warn = "Warn",
    Trace = "Trace",
    Debug = "Debug",
    Fatal = "Fatal"
};



export async function writeLog(code: LogLevel, msg: string) {
    // There is no such thing as an append for some reason...
    let contents = await readTextFile('log.log', { dir: BaseDirectory.AppData });
    contents = `${contents}\nFrontend: Level: ${code} - Message: ${msg}`


    await writeTextFile({
        path: 'log.log',
        contents: contents
    },
        {
            dir: BaseDirectory.AppData
        }
    )
}