import { createRouter, createWebHashHistory } from "vue-router"

export const routes = [
  {
    path: '/',
    redirect: '/yolo/v9',
  },
  {
    path: '/yolo',
    name: 'YOLO',
    meta: {
      title: 'YOLO'
    },
    children: [{
      path: '/yolo/v9',
      name: 'YOLOV9',
      component: () => import('@/pages/YOLO/V9.vue'),
      meta: {
        icon: 'Food',
        title: 'YOLOV9'
      }
    }]
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router;
