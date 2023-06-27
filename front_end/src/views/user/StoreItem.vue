<template>
    <main >
        <div class="cover" @click="router.push('/store/detail/'+data.storeId)">
            <img :src="proxy.globalInfo.storeCoverUrl+data.storeId" alt="">
        </div>
        <div class="name">{{ data.name}}</div>
        <button @click="router.push('/store/detail/'+data.storeId)">进店看看</button>
    </main>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { getCurrentInstance } from 'vue'
const { proxy } = getCurrentInstance()
const props = defineProps({
    data: {
        type: Object,
        default: () => { }
    }
})
const router = useRouter()
</script>

<style scoped lang="scss">
main {
    border-radius: 20px;
    background-size: cover;
    overflow: hidden;
    display: flex;
    padding: 20px;
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
    .cover {
        cursor: pointer;
        height: 100px;
        width: 100px;
        border-radius: 10px;
        overflow: hidden;

        img {
            width: 100%;
            height: 100%;
            object-fit: cover;
        }
    }

    .name {
        margin-left: 30px;
				font-size: 30px;
				display: flex;
				align-items: center;
				letter-spacing: 10px;
				position: relative;
				// cursor: none;
				&::after {
					transition: all 0.3s ease;
					content: '';
					position: absolute;
					background-color: var(--main-color-purple);
					height: 3px;
					width: 0;
					bottom: 20px;
					left: -4px;
				}
				&:hover {
					&::after {
						width: 100%;
					}
				}
    }

    button {
        margin-left: auto;
        width: 150px;
        background-color: var(--main-color-purple);
        border-radius: 10px;
        border: none;
        font-size: 20px;
        color: white;
        cursor: pointer;

        &:hover {
            background-color: var(--main-color-purple-hover);
        }
    }
}
</style>