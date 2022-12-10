import Vue from "vue";
import { store, vuetifyRestoreHook } from "@/plugins/store";
import { vuetify } from "@/plugins/vuetify";

import App from "@/layouts/EmptyLayout.vue";
import { router } from "@/pages/login/router";

new Vue({
  store,
  router,
  vuetify,
  beforeCreate: vuetifyRestoreHook,
  render: h => h(App)
}).$mount("#app");
