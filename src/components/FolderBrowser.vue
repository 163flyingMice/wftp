<template>
    <a-modal :visible="folderBrowserVisible" title="浏览文件夹" centered cancelText="取消" okText="确定"
        @cancel="folderBrowserVisible = false" :maskClosable="false" @ok="handleOk">
        <a-row class="button">
            <a-select v-model:value="value" :options="options" mode="tags" size="middle" placeholder="Please select"
                style="width: 400px" @deselect="deselect">
            </a-select>
        </a-row>
        <div style="overflow: auto;height: 300px;">
            <a-row class="button">
                <a-col :span="6" v-for="(elem, key) in folderList" :key="key" @dblclick="handleClick(elem)">
                    <a-row class="button">
                        <a-col :span="24">
                            <folder-open-outlined :style="{ fontSize: '400%' }" />
                        </a-col>
                    </a-row>
                    <a-row class="button">
                        <a-col :span="24">
                            {{ elem.name }}
                        </a-col>
                    </a-row>
                </a-col>
            </a-row>
        </div>
    </a-modal>
</template>
<script>
import store from '@/store';
import {
    FolderOpenOutlined,
} from "@ant-design/icons-vue";
import {
    readDir,
} from "@tauri-apps/api/fs";

export default {
    components: {
        FolderOpenOutlined,
    },
    setup() {
    },
    mounted() {
        this.getData();
    },
    computed: {
        folderBrowserVisible: {
            get() {
                return store.state.folderBrowserVisible;
            },
            set(value) {
                store.state.folderBrowserVisible = value;
            },
        },
    },
    methods: {
        getData() {
            this.value = [];
            this.options = this.currentPath.split("/").map((elem, index) => {
                if (index == 0) {
                    elem = "/"
                }
                if (elem != "") {
                    this.value.push(elem)
                }
                if (index == this.currentPath.split("/").length - 1) {
                    return { value: elem, disabled: false }
                }
                return { value: elem, disabled: true }
            })
            readDir(this.currentPath).then((response) => {
                let folder_list = [];
                response.forEach((elem) => {
                    if (elem.children != undefined) {
                        let temp = {};
                        temp.name = elem.name;
                        temp.path = elem.path.replaceAll("\\", "/");
                        folder_list.push(temp);
                    }
                });
                this.folderList = folder_list;
            });
        },
        deselect() {
            this.value.shift();
            this.currentPath = this.value.join("/")
            this.currentPath = "/" + this.currentPath
            this.getData();
        },
        handleClick(elem) {
            this.currentPath = elem.path
            this.getData();
        },
        handleOk() {
            store.state.folderBrowserVisible = false;
            store.state.folderBrowserDirectory = this.currentPath;
        }
    },
    data() {
        return {
            currentPath: "/wftp",
            folderList: [],
            value: [],
            options: [],
            fileList: [],
            labelType: "",
            name: "",
        }
    },

};
</script>