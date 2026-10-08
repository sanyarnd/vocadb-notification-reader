import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import "@/styles/main.css";

import { createPinia } from "pinia";
import { createApp } from "vue";

import App from "@/App.vue";
import { setUnauthorizedHandler } from "@/api";
import { createAppVuetify } from "@/plugins/vuetify";
import { createAppRouter } from "@/router";
import { useAccountStore } from "@/stores/account";

const app = createApp(App);
const pinia = createPinia();
const router = createAppRouter();

app.use(pinia);
app.use(router);
app.use(createAppVuetify());

setUnauthorizedHandler(() => {
  useAccountStore(pinia).logout();
  void router.push({ name: "login" });
});

app.mount("#app");

const account = useAccountStore(pinia);
if (account.isAuthenticated) {
  // Ended sessions are handled by the unauthorized handler, other failures are transient.
  account.refresh().catch(() => {});
}
