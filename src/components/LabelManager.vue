<template>
    <a-modal :visible="false" title="新建书签" centered>
        <a-form-item label="类型">
            <a-radio-group v-model:value="formState.resource">
                <a-radio value="1">Sponsor</a-radio>
                <a-radio value="2">Venue</a-radio>
            </a-radio-group>
        </a-form-item>
    </a-modal>
</template>
<script>
export default {
    setup() {
    },

};
</script>