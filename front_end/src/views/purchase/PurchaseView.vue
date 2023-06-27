<template>
    <main>
        <header>下单</header>
        <div v-if="!purchaseSuccess" class="content">
            <div class="address">
                <div class="label"><span>请选你的收货地址</span></div>
                <!-- <el-cascader :options="options"
                v-model="address"></el-cascader> -->
                <el-radio-group v-model="selectedAddress" v-if="addressList.length>0">
                    <div v-for="(item, index) in addressList" :key="index" class="add-item">
                        <el-radio default :label="item" :model-value="item">{{ item.address + ' ' + item.phone }}</el-radio>
                    </div>
                </el-radio-group>
                <div class="add">
                    <el-button type="primary" @click="addAddress">使用 新地址</el-button>
                </div>
            </div>
            <div class="detail">
                <div class="label"><span>确认订单信息</span></div>
                <div class="info">
                    <div class="left">
                        <span>商品</span>
                    </div>
                    <span>单价</span>
                    <span>数量</span>
                    <span>小计</span>
                </div>
                <div class="list">

                    <PurchaseListItem class="list-item" v-for="(item, index) in purchaseListStore.list" :key="index"
                        :data="item"></PurchaseListItem>
                </div>
                <div class="all">
                    <div class="all-wrap">
                        <div class="all-info">
                            <div class="total-price">实付款¥<span>{{ totalPrice.toFixed(2) }}</span></div>
                            <div class="address-info">寄送至: <span v-if="selectedAddress">{{ selectedAddress.address+'  ' +selectedAddress.phone }}</span></div>
                        </div>
                        <div class="op">
                            <div class="back" @click="router.go(-1)">返回购物车</div>
                            <div class="submit" @click="submit">提交订单</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <el-result
        v-else
        icon="success"
        title="购买成功"
        :sub-title="timeout + '秒后返回购物车'"
      >
        <template #extra>
          <el-button type="primary" @click="router.replace('/user')">立即返回</el-button>
        </template>
      </el-result>
      <DialogModule :show="dialogConfig.show" :title="dialogConfig.title" :buttons="dialogConfig.buttons"
            :showCancel="false" @close="dialogConfig.show = false">
            <div class="dialog-body">
                <el-form ref="formDataRef" :model="formData" :rules="rules" label-position="top">
                    <el-form-item label="手机号" prop="phone">
                        <el-input v-model="formData.phone"></el-input>
                    </el-form-item>
                    <el-form-item label="所在地区" prop="area">
                        <el-cascader :options="options" v-model="formData.area"></el-cascader>
                    </el-form-item>
                    <el-form-item label="详细地址" prop="detailAddress">
                        <el-input v-model="formData.detailAddress"></el-input>
                    </el-form-item>
                    <el-checkbox v-model="formData.setDefault" label="设置为默认地址"></el-checkbox>
                </el-form>
            </div>
        </DialogModule>
    </main>
</template>

<script setup>
import { ref, reactive, onMounted, onBeforeUnmount, watch } from 'vue'
import { usePurchaseListStore } from '@/stores/purchaseList'
import { parseAddressCodeArr } from '@/utils/tools'
import DialogModule from '@/components/DialogModule.vue'
import PurchaseListItem from './PurchaseListItem.vue'
import { useRouter } from 'vue-router'
import message from '@/utils/message'
const purchaseListStore = usePurchaseListStore()
import { regionData } from 'element-china-area-data'
const options = regionData
import { getDetailUserInfo, editUserInfo } from '@/api/user'
import { immediatePurchase } from '@/api/purchase'
const formDataRef = ref(null)
const formData = ref({})
const totalPrice = ref(0)
const router = useRouter()

const purchaseSuccess = ref(false)
const timeout = ref(5)

purchaseListStore.list.forEach(item => {
    totalPrice.value += item.num * item.price
})
const addressList = ref([])
const detailUserInfo = ref({})
// 请求获取用户默认地址
onMounted(async () => {
    if (purchaseListStore.list.length === 0){
        router.replace('/user/shoppingCart')
        return
    }
    const res = await getDetailUserInfo()
    if (!res) return
    detailUserInfo.value = res.data
    if (!res.data.address)
        return
    detailUserInfo.value.address = JSON.parse(res.data.address)
    
    let address = detailUserInfo.value.address.areaArr.join('  ')
    addressList.value.push({
        address,
        phone: res.data.phone
    })
    console.log(addressList.value)
})
watch(purchaseListStore.list, (newVal) => {
    totalPrice.value = 0
    newVal.forEach(item => {
        totalPrice.value += item.num * item.price
    })
}, {immediate: true})
onBeforeUnmount(() => {
    purchaseListStore.list = []  
})
const selectedAddress = ref(addressList.value[0])
const dialogConfig = reactive({
    show: false,
    title: '添加新地址',
    buttons: [
        {
            text: '取消',
            type: 'default',
            click: () => {
                dialogConfig.show = false
            }
        },
        {
            text: '确认',
            type: 'primary',
            click: () => {
                formDataRef.value.validate((valid) => {
                    if (valid) {

                        addressList.value.push({
                            address: parseAddressCodeArr(formData.value.area).join(' ')
                                + ' ' + formData.value.detailAddress + '  ',
                            phone: formData.value.phone
                        }
                        )
                        dialogConfig.show = false
                        if (formData.value.setDefault) {
                            detailUserInfo.value.address = {
                                codeArr: formData.value.area,
                                areaArr: parseAddressCodeArr(formData.value.area),
                                detailAddress: formData.value.detailAddress
                            }
                            detailUserInfo.value.phone = formData.value.phone
                            const uploadformData = new FormData()
                            uploadformData.append('address', JSON.stringify(detailUserInfo.value.address))
                            uploadformData.append('phone', detailUserInfo.value.phone)
                            uploadformData.append('userName', detailUserInfo.value.userName)
                            uploadformData.append('gender', detailUserInfo.value.sex)
                
                            editUserInfo(uploadformData)
                        }
                    } else {
                        return false
                    }
                })
            }
        }
    ]
})
const addAddress = () => {
    dialogConfig.show = true
}

const rules = {
    phone: [
        { required: true, message: '请输入手机号', trigger: 'blur' },
        { pattern: /^1[3456789]\d{9}$/, message: '手机号格式不正确', trigger: 'blur' }
    ],
    area: [
        { required: true, message: '请选择所在地区', trigger: 'blur' }
    ],
    detailAddress: [
        { required: true, message: '请输入详细地址', trigger: 'blur' }
    ]
}

const submit = async () => {
    if (!selectedAddress.value) {
        message.warning('请选择一个收货地址')
        return
    }
    const list = purchaseListStore.list.map(item => {
        return {
            productId: item.productId,
            num: item.num.toString()
        }
    })
    const data = {
        address: selectedAddress.value.address,
        phone : selectedAddress.value.phone,
        list
    }
    const res = await immediatePurchase(data)
    if (!res) return
    purchaseListStore.list = []
    message.success('提交订单成功')

    purchaseSuccess.value = true
    const id = setInterval(() => {
        timeout.value--
        if (timeout.value === 0) {
            clearInterval(id)
            router.replace('/user')
            
        }
    }, 1000)
    // 发送请求提交订单
}
</script>

<style lang="scss" scoped>
main {
    margin: 40px auto;
    width: var(--global-width);
    background-color: white;
    border-radius: 20px;
    overflow: hidden;

    header {
        display: flex;
        align-items: center;
        font-size: 22px;
        padding: 20px;
        border-bottom: 1px solid #eee;
        justify-content: space-between;
    }

    .content {
        padding: 20px;

        // display: flex;
        .address {

            .add-item {
                margin-left: 20px;
                margin-bottom: 10px;
                display: flex;
                width: 100%;
            }

            .add {
                margin-top: 20px;
                margin-left: 20px;
            }
        }

        .detail {
            margin-top: 40px;

            .list {
                .list-item {
                    margin-top: 20px;
                }
            }

            .info {
                display: flex;

                // justify-content: space-between;
                span {
                    flex: 1;
                    border-bottom: 3px solid var(--main-color-purple);
                    margin-right: 10px;
                    padding-bottom: 5px;
                }

                .left {
                    width: 400px;
                }
            }
        }

        .all {
            margin-top: 40px;
            display: flex;
            justify-content: end;

            .all-wrap {
                border: 2px solid #ffa9a9;
                border-radius: 10px;
                padding: 10px;
                min-width: 400px;

                .all-info {
                    display: flex;
                    flex-direction: column;

                    .total-price {
                        align-self: flex-end;
                        // display: flex;
                        // justify-content: space-between;
                        font-size: 18px;
                        margin-bottom: 10px;

                        span {
                            margin-left: 10px;
                            font-weight: 600;
                            font-size: 30px;
                            color: rgb(255, 116, 116);
                        }
                    }

                    .address-info {
                        margin-top: 10px;
                        font-weight: 600;
                    }
                }

                .op {
                    margin-top: 20px;
                    display: flex;
                    justify-content: end;
                    align-items: center;

                    .back {
                        color: var(--main-color-purple);
                        cursor: pointer;

                        &:hover {
                            text-decoration: underline;
                            color: rgb(255, 116, 116);
                        }
                    }

                    .submit {
                        margin-left: 20px;
                        padding: 10px 20px;
                        border-radius: 10px;
                        color: white;
                        cursor: pointer;
                        background-color: rgb(255, 159, 175);
                        transition: all 0.3s;

                        &:hover {
                            background-color: rgb(255, 116, 116);
                        }
                    }
                }
            }
        }

        .left {
            display: flex;
            align-items: start;
        }

        .label {
            font-size: 18px;
            margin-bottom: 20px;

            span {
                padding-bottom: 10px;
                cursor: pointer;
                position: relative;

                &::after {
                    position: absolute;
                    content: '';
                    width: 0;
                    height: 2px;
                    background-color: var(--main-color-purple);
                    bottom: 0px;
                    left: 0;
                }

                &:hover {
                    &::after {
                        width: 100%;
                        transition: width 0.3s;
                    }
                }
            }
        }
    }
}</style>