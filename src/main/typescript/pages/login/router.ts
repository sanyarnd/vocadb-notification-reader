import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";

Vue.use(VueRouter);

const routes: RouteConfig[] = [
  {
    path: "/",
    name: "login",
    component: () => import("@/pages/login/views/Login.vue")
  }
];

export const router = new VueRouter({
  base: "/login",
  mode: "history",
  routes: routes
});
