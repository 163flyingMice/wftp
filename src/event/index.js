import store from '@/store';
import { invoke } from '@tauri-apps/api';
import {
    listen
} from '@tauri-apps/api/event';

export function definedListeningEvent() {
    listen('site_manager', () => {
        store.state.modalVisible = true;
    });
    listen('refresh_page', () => {
        window.location.reload();
    });
    listen('add_label', () => {
        store.state.addLableVisible = true;
    });
    listen('label_manager', () => {
        store.state.labelManagerVisible = true;
    });
    listen('add_current_to_site', () => {
        store.state.addCurrentToSite = true;
        store.state.modalVisible = true;
    });
    listen('erase_personal_information', () => {
        localStorage.clear();
        invoke("get_default_wftp", {}).then((response) => {
            let res = JSON.parse(response);
            if (res.code == 200) {
                localStorage.setItem("wftp_server", res.list);
                invoke("get_wftp_server", { wftpXml: res.list }).then(
                    (response) => {
                        let res = JSON.parse(response);
                        if (res.code == 200) {
                            store.state.wftpServer = res.list;
                        }
                    }
                );
            }
        });
    });

}