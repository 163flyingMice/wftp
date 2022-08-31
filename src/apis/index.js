import store from "@/store";
import {
    invoke
} from "@tauri-apps/api";

let connected;

let funcMap = {
    "connect": ["sftp_connect", "connect"],
    "readdir": ["readdir", "list"],
    "getTree": ["sftp_folder_list", "folder_list"],
    "cwd": ["sftp_cwd", "cwd"],
    "prev": ["sftp_prev", "prev"],
    "create": ["sftp_create", "mk_file"],
    "unlink": ["sftp_unlink", "remove_file"],
    "rmdir": ["sftp_rmdir", "remove_dir"],
    "mk_dir": ["sftp_mkdir", "mk_dir"],
    "pwd": ["sftp_pwd", "pwd"],
    "rename_file": ["sftp_rename", "rename_file"],
    "size_sort": ["size_sort", "size_sort"],
    "upload": ["upload", "upload"],
};


export async function connect() {
    connected = getProtocol();
    store.state.stateList.push("状态：正在连接" + connected.data.Name);
    connected.connected = false;
    invoke(funcMap["connect"][connected.data.Protocol], {
        addr: connected.data.Host + ":" + connected.data.Port,
        user: connected.data.User,
        pass: connected.data.Pass,
    }).then((response) => {
        let res = JSON.parse(response);
        if (res.code == 200) {
            store.state.connected = true;
            connected.connected = true;
            connected.connectedId = res.list;
            store.state.stateList.push("响应：" + res.msg);
        } else {
            store.state.stateList.push("响应：" + res.msg);
        }
    }).catch((err) => {
        store.state.stateList.push("响应：" + err);
    });
}

export async function readdir(path) {
    connected = getProtocol();
    return await invoke(funcMap["readdir"][connected.data.Protocol], {
        name: connected.connectedId,
        path: path,
    })
}

export async function getTree() {
    connected = getProtocol();
    return await invoke(funcMap["getTree"][connected.data.Protocol], {
        name: connected.connectedId,
    })
}

export async function cwd(path) {
    connected = getProtocol();
    return await invoke(funcMap["cwd"][connected.data.Protocol], {
        name: connected.connectedId,
        path: path,
    })
}

export async function prev() {
    connected = getProtocol();
    return await invoke(funcMap["prev"][connected.data.Protocol], {
        name: connected.connectedId,
    })
}

export async function create(filename) {
    connected = getProtocol();
    return await invoke(funcMap["create"][connected.data.Protocol], {
        name: connected.connectedId,
        filename: filename
    })
}

export async function unlink(filename) {
    connected = getProtocol();
    return await invoke(funcMap["unlink"][connected.data.Protocol], {
        name: connected.connectedId,
        filename: filename
    })
}

export async function rmdir(path) {
    connected = getProtocol();
    return await invoke(funcMap["rmdir"][connected.data.Protocol], {
        name: connected.connectedId,
        path: path
    })
}

export async function mk_dir(path) {
    connected = getProtocol();
    return await invoke(funcMap["mk_dir"][connected.data.Protocol], {
        name: connected.connectedId,
        path: path
    })
}

export async function pwd() {
    connected = getProtocol();
    return await invoke(funcMap["pwd"][connected.data.Protocol], {
        name: connected.connectedId
    })
}

export async function rename_file(fromName, toName) {
    connected = getProtocol();
    return await invoke(funcMap["rename_file"][connected.data.Protocol], {
        name: connected.connectedId,
        fromName: fromName,
        toName: toName
    })
}

export async function size_sort(fileList, sortWay) {
    connected = getProtocol();
    return await invoke(funcMap["size_sort"][connected.data.Protocol], {
        name: connected.connectedId,
        fileList: fileList,
        sortWay: sortWay
    })
}

export async function upload(filename, content) {
    connected = getProtocol();
    return await invoke(funcMap["upload"][connected.data.Protocol], {
        name: connected.connectedId,
        filename: filename,
        content: content,
    })
}



export function getProtocol() {
    store.state.panes.forEach((elem) => {
        if (elem.key == store.state.listActiveKey) {
            connected = elem;
        }
    });
    return connected
}