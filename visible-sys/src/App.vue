<template>
  <div class="flex flex-row h-100%">
    <div class="w-50">
      <el-menu class="h-100%" router>
        <MenuTree :menuData="menuData" />
      </el-menu>
    </div>
    <div class="flex-1 h-100% pa-5">
      <router-view />
    </div>
  </div>
</template>

<script setup lang="ts">
import { routes } from '@/router'
const menuData = routes.splice(1)
</script>
