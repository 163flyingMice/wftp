import store from '@/store';
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
}