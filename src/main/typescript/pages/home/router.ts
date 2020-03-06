import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";

Vue.use(VueRouter);

const routes: RouteConfig[] = [
  {
    path: "/",
    name: "home",
    component: () => import("@/pages/home/views/Home.vue")
  }
];

export const router = new VueRouter({
  base: "/home",
  mode: "history",
  routes: routes
});
