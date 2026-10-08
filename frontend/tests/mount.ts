import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import type { Component } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

import { messages } from "@/i18n";

export function createTestRouter() {
  return createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: "/", name: "home", component: { template: "<div>home</div>" } },
      { path: "/login", name: "login", component: { template: "<div>login</div>" } }
    ]
  });
}

export function mountWithPlugins(component: Component, options: { props?: object } = {}) {
  const pinia = createPinia();
  setActivePinia(pinia);
  const router = createTestRouter();
  const vuetify = createVuetify({
    components,
    directives,
    locale: { locale: "en", fallback: "en", messages }
  });

  const wrapper = mount(component, {
    props: options.props,
    attachTo: document.body,
    global: { plugins: [pinia, router, vuetify] }
  });
  return { wrapper, router, pinia };
}
