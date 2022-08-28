import {
    createStore
} from 'vuex'

export default createStore({
    /**
     * 集中存储组件的状态
     */
    state: {
        stateList: [],
        transfeList: [{
            'localName': '123',
            'size': '10000',
            'direction': '->',
            'remoteName': '',
            'priority': '高',
            'time': '456',
        }, {
            'localName': '',
            'size': '10000',
            'direction': '<-',
            'remoteName': '456',
            'priority': '高',
            'time': '456',
        }, {
            'localName': '',
            'size': '10000',
            'direction': '<-',
            'remoteName': '456',
            'priority': '高',
            'time': '456',
        }, {
            'localName': '',
            'size': '10000',
            'direction': '<-',
            'remoteName': '456',
            'priority': '高',
            'time': '456',
        }],
        connected: false,
        wftpServer: [],
        localSiteComponent: true,
        remoteSiteComponent: true,
        stateListComponent: true,
        connectedName: "",
        modalVisible: false,
        panes: [],
        addLableVisible: false,
        folderBrowserVisible: false,
        folderBrowserDirectory: "",
        listActiveKey: "1",
        transfeListComponent: true,
        labelManagerVisible: false,
    },
    /**
     * 改变数据方法的集合
     */
    mutations: {},
    /**
     * 改变mutations，不直接变更状态
     */
    actions: {},
    /**
     * 可以将store进行功能拆分，分割成不同的模块
     */
    modules: {}
})