<template>
  <div class="hello">
    <div>
        Login: <input type="text" />
        Password: <input type="password" />
        <input type="submit" />
    </div>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
  props: {
    msg: String
  }
})

export default class Login extends Vue {
  msg!: string
}
</script>

<style scoped lang="scss">

</style>