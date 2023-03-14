import { createRouter, createWebHistory } from "vue-router";
import Home from "@/views/Home.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: Home,
    },
    {
      path: "/tea/:slug",
      component: () =>
        import(/* webpackChunkName: "tea-details" */ "@/views/TeaDetails.vue"),
    },
    {
      path: "/:pathMatch(.*)*",
      redirect: "/",
    },
  ],
});
