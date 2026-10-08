import { createRouter, createWebHashHistory, type Router } from "vue-router";

import { useAccountStore } from "@/stores/account";

declare module "vue-router" {
  interface RouteMeta {
    requiresAuth?: boolean;
  }
}

export function createAppRouter(): Router {
  // Hash history keeps the bundle deployable to a plain static bucket/CDN.
  const router = createRouter({
    history: createWebHashHistory(),
    routes: [
      {
        path: "/",
        name: "home",
        component: () => import("@/views/HomeView.vue"),
        meta: { requiresAuth: true }
      },
      {
        path: "/login",
        name: "login",
        component: () => import("@/views/LoginView.vue")
      },
      { path: "/:pathMatch(.*)*", redirect: "/" }
    ]
  });

  router.beforeEach(to => {
    const account = useAccountStore();
    if (to.meta.requiresAuth && !account.isAuthenticated) {
      return { name: "login" };
    }
    if (to.name === "login" && account.isAuthenticated) {
      return { name: "home" };
    }
    return true;
  });

  return router;
}
