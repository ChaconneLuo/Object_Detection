<template>
  <div class="flex flex-row h-100%">
    <div class="w-50" v-if="!route.path.includes('login')">
      <el-menu class="h-100%" router>
        <MenuTree :menuData="menuData" />
      </el-menu>
    </div>
    <div class="flex-1 h-100%" :class="!route.path.includes('login') ? 'p-5' : ''">
      <router-view />
    </div>
  </div>
</template>

<script setup lang="ts">
import { routes } from '@/router'
import { useRoute } from 'vue-router'
const route = useRoute()
const menuData = routes.splice(2)
</script>
