import store from "@/store";
import { invoke } from "@tauri-apps/api";

let connected;


export function connect() {
    let execFunc;
    store.state.panes.forEach((elem) => {
        if (elem.key == store.state.listActiveKey) {
            connected = elem;
        }
    });
    switch (connected.data.Protocol) {
        case 1:
            execFunc = "connect"
            break;
        default:
            execFunc = "sftp_connect"
            break;
    }
    invoke(execFunc, {
        name: connected.title,
        addr: connected.data.Host + ":" + connected.data.Port,
        user: connected.data.User,
        pass: connected.data.Pass,
    }).then((response) => {
        let res = JSON.parse(response);
        if (res.code == 200) {
            store.state.connected = true;
        }
        store.state.stateList.push("响应：" + res.msg);
    }).catch((err) => {
        store.state.stateList.push("响应：" + err);
    });
}

export async function readdir(path) {
    let execFunc;
    switch (connected.data.Protocol) {
        case 1:
            execFunc = "list"
            break;
        default:
            execFunc = "readdir"
            break;
    }
    return await invoke(execFunc, {
        name: connected.data.Name,
        path: path,
    })
}

export async function getTree() {
    let execFunc;
    switch (connected.data.Protocol) {
        case 1:
            execFunc = "folder_list"
            break;
        default:
            execFunc = "sftp_folder_list"
            break;
    }
    return await invoke(execFunc, {
        name: connected.data.Name,
    })
}

export async function cwd(path) {
    let execFunc;
    switch (connected.data.Protocol) {
        case 1:
            execFunc = "cwd"
            break;
        default:
            execFunc = "sftp_cwd"
            break;
    }
    return await invoke(execFunc, {
        name: connected.data.Name,
        path: path,
    })
}

export async function prev() {
    let execFunc;
    switch (connected.data.Protocol) {
        case 1:
            execFunc = "prev"
            break;
        default:
            execFunc = "sftp_prev"
            break;
    }
    return await invoke(execFunc, {
        name: connected.data.Name,
    })
}