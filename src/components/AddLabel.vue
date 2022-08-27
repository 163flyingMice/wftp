<template>
    <a-modal :maskClosable="false" :visible="addLableVisible" title="新建书签" centered cancelText="取消" okText="确定"
        @cancel="addLableVisible = false">
        <a-step title="类型" />
        <a-form>
            <a-form-item>
                <a-radio-group v-model:value="labelType">
                    <a-radio value="1">全局书签</a-radio>
                    <a-radio value="2" disabled>特定站点书签</a-radio>
                </a-radio-group>
            </a-form-item>
        </a-form>

        <a-step title="名字" />
        <a-form>
            <a-form-item label="名字">
                <a-input v-model:value="name" />
            </a-form-item>
        </a-form>
        <a-step title="路径" />
        <a-form layout="inline">
            <a-form-item label="本地目录">
                <a-input v-model:value="localDirectory" />
            </a-form-item>
            <a-form-item>
                <a-button @click="handleClick">
                    浏览
                </a-button>
            </a-form-item>
        </a-form>
        <a-form>
            <a-form-item label="远程目录">
                <a-input v-model:value="name" />
            </a-form-item>
            <a-form-item>
                <a-checkbox name="type">使用同步浏览</a-checkbox>
            </a-form-item>
            <a-form-item>
                <a-checkbox name="type">目录对比</a-checkbox>
            </a-form-item>
        </a-form>
    </a-modal>
</template>
<script>
import store from '@/store';

export default {
    components: {
    },
    setup() {
    },
    methods: {
        handleClick() {
            store.state.folderBrowserVisible = true;
        }
    },
    data() {
        return {
            fileList: [],
            labelType: "",
            name: "",
        }
    },
    computed: {
        addLableVisible: {
            get() {
                return store.state.addLableVisible;
            },
            set(value) {
                store.state.addLableVisible = value;
            },
        },
        localDirectory() {
            return store.state.folderBrowserDirectory;
        }
    },

};
</script>