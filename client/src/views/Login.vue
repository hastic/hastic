<template>
  <div class="hello">
    <div>
        Login: <input type="text" v-model="this.username" /> <br/> <br/>
        Password: <input type="password" v-model="this.password" /> <br/> <br/>
        <input type="submit" @click="this.submit" value="login" />
    </div>
  </div>
</template>

<script lang="ts">
import AuthService from '@/services/auth.service';

import { Options, Vue } from 'vue-class-component';

@Options({
  
})

export default class Login extends Vue {
  msg!: string

  username: string;
  password: string;

  submit() {
    AuthService.login({ username: this.username, password: this.password  })
  }
}
</script>

<style scoped lang="scss">

</style>