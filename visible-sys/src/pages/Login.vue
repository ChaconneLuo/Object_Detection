<template>
  <div class="h-100vh flex justify-center items-center background">
    <div class="w-300px h-250px">
      <el-form ref="formRef" style="max-width: 600px" :model="loginForm" label-width="auto">
        <el-form-item prop="username" label="用户名">
          <el-input v-model="loginForm.username" />
        </el-form-item>
        <el-form-item prop="password" label="密码">
          <el-input v-model="loginForm.password" />
        </el-form-item>
        <el-form-item>
          <div class="w-100% flex justify-center">
            <el-button type="primary" @click="login">登录</el-button>
            <el-button @click="resetForm(formRef)">清除</el-button>
          </div>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import type { FormInstance } from 'element-plus'
import { reactive, Ref, ref } from 'vue'
import { useRouter } from 'vue-router'
const router = useRouter()

const formRef = ref<FormInstance>()
const resetForm = (formRef: Ref<FormInstance>) => {
  formRef.value?.resetFields()
}
const loginForm = reactive({
  username: '',
  password: '',
})

const login = async () => {
  const response = JSON.parse(await invoke('login', { username: loginForm.username, password: loginForm.password }))
  if (response.res === 200) {
    router.push('/yolo/v9')
  }
}
</script>

<style scoped>
.background {
  background: url('../assets/background.jpg') no-repeat center center;
  background-size: cover;
}

:deep(.el-form-item__label) {
  color: white;
}
</style>
