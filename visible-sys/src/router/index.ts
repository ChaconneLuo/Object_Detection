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
    }, {
      path: '/yolo/v8',
      name: 'YOLOV8',
      component: () => import('@/pages/YOLO/V8.vue'),
      meta: {
        icon: 'Apple',
        title: 'YOLOV8'
      }
    }, {
      path: '/yolo/v5',
      name: 'YOLOV5',
      component: () => import('@/pages/YOLO/V5.vue'),
      meta: {
        icon: 'Burger',
        title: 'YOLOV5'
      }
    }]
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router;
