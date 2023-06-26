import { ElMessage } from 'element-plus'
import { ElMessageBox } from 'element-plus'
const showMessage = (msg, callback, type) => {
    ElMessage({
        type,
        message: msg,
        duration: 2000,
        onClose: () => {
            if (callback) {
                callback()
            }
        }
    })
}
const showConfirm = (msg, callback) => {
    ElMessageBox.confirm(msg, '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'info'
    }).then(() => {
        callback()
    }).catch(() => {
    })
}

const message = {
    error: (msg, callback) => {
        showMessage(msg, callback, 'error')
    },
    success: (msg, callback) => {
        showMessage(msg, callback, 'success')
    },
    warning: (msg, callback) => {
        showMessage(msg, callback, 'warning')
    },
    confirm: (msg, callback) => {
        showConfirm(msg, callback)
    }
}
export default message
