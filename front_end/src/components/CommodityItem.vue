<template>
  <main>
    <div class="cover">
      <img
        :src="data.cover"
        alt=""
      >
    </div>
    <div class="info">
      <div class="name">
        {{ data.name }}
      </div>
      <div class="price">
        ¥ {{ data.price }}
      </div>
    </div>
  </main>
</template>

<script setup>
const props = defineProps({
    data: {
        type: Object,
        default: {}
    }
})
</script>

<style scoped lang="scss">
main{
    max-width: 340px;
    height: 180px;
    display: flex;
    justify-content: space-between;
    padding: 10px;
    background-color: #f1f5fa;
    border-radius: 10px;
    transition: all 0.3s;
    border: solid 1px transparent;
    cursor: pointer;
    &:hover{
        background-color: white;
        border: 1px solid var(--main-color-purple);
    }
    .cover{
        width: 160px;
        height: 160px;
        img{
            width: 100%;
            height: 100%;
            object-fit: cover;
            border-radius: 10px;
        }
    }
    .info {
        margin-top: 10px;
        margin-right: 10px;
        width: 130px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        .name{
            overflow-wrap: break-word;
            font-size: 16px;
            transition: all 0.2s;
            
            &:hover{
                color:rgb(255, 106, 106)
            }
        }
        .price{
            font-size: 24px;
            font-weight: 500;
            color: rgb(255, 106, 106);
        }
    }
}
</style>