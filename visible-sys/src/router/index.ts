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
      path: '/yolo/v5s',
      name: 'YOLOV5s',
      component: () => import('@/pages/YOLO/V5s.vue'),
      meta: {
        icon: 'Burger',
        title: 'YOLOV5s'
      }
    }]
  },
  {
    path: '/compare',
    name: 'Compare',
    component: () => import('@/pages/Compare.vue'),
    meta: {
      title: 'Compare'
    },
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router;
