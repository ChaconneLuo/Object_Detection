import { createRouter, createWebHashHistory } from 'vue-router'

export const routes = [
  {
    path: '/',
    redirect: '/login',
  },
  {
    path: '/login',
    component: () => import('@/pages/Login.vue'),
  },
  {
    path: '/yolo',
    name: 'YOLO',
    meta: {
      title: 'YOLO训练数据可视化',
    },
    children: [
      {
        path: '/yolo/v9',
        name: 'YOLOV9',
        component: () => import('@/pages/YOLO/V9.vue'),
        meta: {
          icon: 'Food',
          title: 'YOLOV9',
        },
      },
      {
        path: '/yolo/v8',
        name: 'YOLOV8',
        component: () => import('@/pages/YOLO/V8.vue'),
        meta: {
          icon: 'Apple',
          title: 'YOLOV8',
        },
      },
      {
        path: '/yolo/v5',
        name: 'YOLOV5',
        component: () => import('@/pages/YOLO/V5.vue'),
        meta: {
          icon: 'Burger',
          title: 'YOLOV5',
        },
      },
    ],
  },
  {
    path: '/compare',
    name: 'Compare',
    component: () => import('@/pages/Compare.vue'),
    meta: {
      title: '识别效果比对',
    },
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
