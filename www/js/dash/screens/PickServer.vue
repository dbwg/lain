<template>
  <div class="section">
    <div class="container has-text-centered">
      <h1 class="title">Pick a server</h1>
      <div class="columns">
        <div class="column is-narrow" v-for="server in servers">
          <router-link :to="{name: 'server', params: {id: server.id}}">
            <img class="server" :src="server.avy" />
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data: function () {
    return {
      servers: [
        {avy: "http://via.placeholder.com/150x150", id: 1337}
      ]
    }
  }
}
</script>

<style lang="scss" scoped>
.columns {
  justify-content: center;
}
.server {
  border-radius: 50%;
  border: 2px solid #eee;
  &:hover {
    border: 2px solid goldenrod;
  }
}
</style>
