<template>
  <div class="flex flex-col">
    <div></div>
    <div class="flex flex-row">
      <el-row class="flex-items-stretch">
        <el-col :span="8" :key="index" v-for="(item, index) of resultData">
          <el-card class="h-100%">
            <template #header>{{ item.name }}</template>
            <img :src="item.img" :alt="item.name" style="width: 100%" />
          </el-card>
        </el-col>
      </el-row>
    </div>
  </div>
</template>
<script setup lang="ts">
const resultData = [
  {
    name: 'confusion_matrix',
    img: '/confusion_matrix.png',
  },
  {
    name: 'F1_curve',
    img: '/F1_curve.png',
  },
  {
    name: 'labels_correlogram',
    img: '/labels_correlogram.jpg',
  },
  {
    name: 'labels',
    img: '/labels.jpg',
  },
  {
    name: 'P_curve',
    img: '/P_curve.png',
  },
  {
    name: 'PR_curve',
    img: '/PR_curve.png',
  },
  {
    name: 'R_curve',
    img: '/R_curve.png',
  },
  {
    name: 'results',
    img: '/results.png',
  },
].map((item) => ({
  ...item,
  img: '../../data/yolov8' + item.img,
}))

</script>
