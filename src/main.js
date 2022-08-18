import { createApp } from 'vue';
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/antd.css';
import App from './App.vue';
import VueClipboard from 'vue-clipboard2'
createApp(App).use(Antd).use(VueClipboard).mount('#app')
