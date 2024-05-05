<template>
  <div>
    <template v-for="(item, index) in menuData" :key="index">
      <el-menu-item v-if="!item.children" :index="item.path">
        <el-icon v-if="item.meta.icon">
          <!-- <House /> -->
          <component :is="item.meta.icon"></component>
        </el-icon>
        <span>{{ item.meta.title }}</span>
      </el-menu-item>
      <el-sub-menu :index="item.path" v-if="item.children">
        <template #title>
          <span>{{ item.meta.title }}</span>
        </template>
        <MenuTree :menuData="item.children"></MenuTree>
      </el-sub-menu>
    </template>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  menuData: any[]
}>()
</script>
