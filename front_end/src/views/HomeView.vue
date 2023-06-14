<template>
  <main @click="closeRecord">
    <header>
      <div class="logo">
        <span>L</span>
        <span>O</span>
        <span>G</span>
        <span>O</span>
      </div>
      <div class="search-box">
        <div
          ref="searchRef" 
          :class="['search', searchOnFocus === true ? 'search-record' : '']"
        >
          <input
            type="text"
            placeholder="请输入你要搜索的关键词"
            @focus="showRecord(true)"
          >
          <div class="btn">
            <el-icon>
              <Search />
            </el-icon>
          </div>
          <div
            v-if="searchOnFocus"
            class="record"
          >
            <p v-if="searchLogs.length > 0">
              搜索历史
            </p>
            <p v-else>
              暂无历史记录
            </p>
            <div
              class="record-list"
            >
              <div
                v-for="(item, index) in searchLogs"
                :key="index"
                class="record-item"
                @click.stop="goSearch(item)"
              >
                {{ item }}
                <span
                  class="del-record"
                  @click.stop="delRecord(index)"
                ><el-icon><Close /></el-icon></span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </header>
    <nav>
      <div class="recommend" />
      <div class="user-info">
        <div class="avatar">
          <AvatarModule 
            :user-id="userInfoStore?userInfoStore.userId:''"
            :size="70"
          />
        </div>
        <div class="info">
          <p>Hi! 你好</p>
          <div class="login">
            <div @click="goAccount('login')">
              登录
            </div>
            <div @click="goAccount('register')">
              注册
            </div>
            <div @click="goAccount('newShop')">
              开店
            </div>
          </div>
          <div class="func">
            <div>
              <el-icon><ShoppingCart /></el-icon>
              <span>购物车</span>
            </div>
            <div>
              <el-icon><Goods /></el-icon>
              <span>买过的店</span>
            </div>
            <div>
              <el-icon><Clock /></el-icon>
              <span>购买记录</span>
            </div>
            <div>
              <el-icon><TakeawayBox /></el-icon>
              <span>我的店铺</span>
            </div>
          </div>
        </div>
      </div>
    </nav>
    <div class="homepage-list">
      <span>猜你喜欢</span>
      <DataList
        :data-source="dataSource"
        @page-no-change="pageNoChange"
      >
        <template #default="{data}">
          <CommodityItem
            :data="data"
            @click="router.push('/commodity/'+data.productId)"
          />
        </template>
      </DataList>
    </div>
  </main>
</template>

<script setup>
import { ref} from 'vue'
import AvatarModule from '@/components/AvatarModule.vue'
import { useUserInfoStore } from '@/stores/userInfo.js'
import { useRouter } from 'vue-router';
import DataList from '@/components/DataList.vue'
import CommodityItem from '@/components/CommodityItem.vue'
const userInfoStore = useUserInfoStore()
const searchRef = ref()
const router = useRouter()
const searchLogs = ref([
    "vue",
    "react",
    "angular",
    "webpack",
    "vite",
    "rollup",
    "babel",
])
const dataSource = {
    pageSize: 20,
    pageCount: 3,
    total: 60,
    list: [
        {
            productId: 1,
            cover: 'https://pic.imgdb.cn/item/64475b180d2dde5777857756.webp',
            name: 'XXxxxxxxxxxxxxXXXXXXXXXXX',
            descriton:'xxxxxxxxxxxxxxxxxxxxx xwewxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
            price: 100
        },
        {
            productId: 1,
            cover: 'https://pic.imgdb.cn/item/64475b180d2dde5777857756.webp',
            name: 'XXxxxxxxxxxxxxXXXXXXXXXXX',
            descriton:'xxxxxxxxxxxxxxxxxxxxx xwewxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
            price: 100
        },
        {
            productId: 1,
            cover: 'https://pic.imgdb.cn/item/63b00e822bbf0e799465cd93.jpg',
            name: 'XXxxxxxxxxxxxxXXXXXXXXXXX',
            descriton:'xxxxxxxxxxxxxxxxxxxxx xwewxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
            price: 100
        }
    ]
}
const searchOnFocus = ref(false)
const showRecord = (flag) => {
    searchOnFocus.value = flag
}
const delRecord = ( index) => {
    searchLogs.value.splice(index, 1)
    console.log('del')
}
const goSearch = ( keyword) => {
    console.log(keyword)
}

const closeRecord = (event) => {
    if (searchRef.value && searchRef.value.contains(event.target)) return
    else searchOnFocus.value = false
}
const goAccount = (type) => {
    router.push('/account/' + type)
}

const pageNoChange = (pageNo) => {
    console.log(pageNo)
}
</script>

<style lang="scss" scoped>
$bg-gray: #ddd;
$main-color: rgb(205, 198, 235);
$main-color-hover: rgb(218, 212, 247);
main {
	// height: 100vh;
	width: var(--global-width);
    margin: 0 auto;

	// background-color: pink;
	header {
		width: 100%;
		display: flex;
		justify-content: space-between;
        padding-top: 20px;

		.logo {
			height: 100px;
			width: 200px;
            display: flex;
            justify-content: center;
            align-items: center;
            padding-left: 20px;
            span{
                font-size: 50px;
                margin-right: 14px;
                font-weight: 600;
                user-select: none;
            }
            span:nth-child(1) {
                color: rgb(255, 154, 154);
            }
            span:nth-child(2) {
                color: rgb(154, 154, 255);
            }
            span:nth-child(3) {
                color: rgb(154, 255, 154);
            }
            span:nth-child(4) {
                color: rgb(255, 255, 187);
            }
		}

		.search-box {
			flex: 1;
			// background-color: aquamarine;
			display: flex;
			justify-content: center;
			align-items: center;
			position: relative;

			.search {
				width: 660px;
				border: 2px solid #ddd;
				border-radius: 14px;
				background-color: #eeeeee;
				display: flex;
				align-items: center;
				justify-content: space-between;
				transition: all 0.3s;
				padding: 4px;

				&:hover {
					background-color: white;
					box-shadow: 0 0 10px #b8b8b8;
				}

			}

			.search-record {
				position: absolute;
				top: calc(50% - 25px);
				border-radius: 14px;
				background-color: white;
				display: flex;
				border: 1px solid $bg-gray;
				box-shadow: 0 0 10px #b8b8b8;
				flex-wrap: wrap;
				transition: none;

				input {
					border-radius: 12px;
					background-color: $bg-gray;
				}

				.record {
					padding: 10px;
          width: 100%;
					p {
						font-size: 16px;
					}

					.record-list {
						margin-top: 10px;
						display: flex;

						.record-item {
							background-color: #e7e7e7;
							padding: 2px 6px;
							border-radius: 4px;
							margin-right: 20px;
							margin-bottom: 20px;
							position: relative;
							&:hover {
								cursor: pointer;
								color: rgb(255, 105, 133);
								background-color: #f0f0f0;
							}
							&:hover .del-record {
								display: flex;
							}
							.del-record {
								position: absolute;
								right: -6px;
								top: -8px;
								display: none;
								justify-content: center;
								align-items: center;
								background-color: $bg-gray;
								color: white;
								border-radius: 50%;
								cursor: pointer;
							}
						}
						
					}
				}
			}

			input {
				height: 40px;
				flex: 1;
				font-size: 16px;
				border: none;
				background-color: transparent;
				padding-left: 20px;
				outline: none;
				border-radius: inherit;
			}

			.btn {
				height: 40px;
				width: 40px;
				font-size: 20px;
				display: flex;
				justify-content: center;
				align-items: center;
				margin-right: 5px;
				margin-left: 5px;

				&:hover {
					cursor: pointer;
					// color: rgb(255, 218, 218);
					background-color: $bg-gray;
					border-radius: 8px;
				}
			}
		}
	}

	nav {
		background-color: white;
		height: 400px;
		border-radius: 20px;
		margin-top: 30px;
		padding: 20px;
		display: flex;
		justify-content: center;
		align-items: center;

		.recommend {
			width: 760px;
			height: 100%;
			background-color: $main-color;
			border-radius: 10px;
		}

		.user-info {
            margin-top: 20px;
			flex: 1;
            height: 100%;
            margin-left: 40px;
            .avatar{
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100px;
                // background-color: gray;
            }
            .info {
                p {
                    width: 100%;
                    text-align: center;
                    margin: 10px auto;
                    font-size: 20px;
                }
                .login {
                    margin-top: 40px;
                    display: flex;
                    justify-content: space-around;
                    align-items: center;
                    div {
                        font-size: 20px;
                        color: white;
                        background-color: $main-color;
                        padding: 6px 18px;
                        border-radius: 20px;
                        cursor: pointer;
                        &:hover{
                            background-color: $main-color-hover;
                        }
                    }
                }
                .func{
                    margin-top: 38px;
                    display: flex;
                    justify-content: space-around;
                    align-items: center;
                    div{
                        font-size: 18px;
                        display: flex;
                        align-items: center;
                        flex-direction: column;
                        cursor: pointer;
                        &:hover{
                            color: $main-color;
                        }
                        span{
                            margin-top: 8px;
                        }
                    }
                }
            }
		}
	}
    .homepage-list {
        margin-top: 30px;
        background-color: white;
        padding: 20px;
        border-radius: 20px;
        span {
            margin-left: 30px;
            font-size: 20px;
            display: block;
            margin-bottom: 20px;
        }
    }
}
</style>