import { ref } from 'vue'

export const useDragResizeStore = () => {
    const isDragging = ref(false)
    const sidebarWidth = ref(20)
    const containerRef = ref(null)
    let rafId = null

    // 节流函数
    function throttleRAF(fn) {
        return (...args) => {
            if (rafId) return
            
            rafId = requestAnimationFrame(() => {
                fn(...args)
                rafId = null
            })
        }
    }

    // 处理拖拽
    const handleDrag = throttleRAF((e) => {
        if (!isDragging.value || !containerRef.value) return
        
        const containerWidth = containerRef.value.offsetWidth
        const newWidth = (e.clientX / containerWidth) * 100
        
        // 限制侧边栏宽度在10%到50%之间
        sidebarWidth.value = Math.min(Math.max(newWidth, 10), 50)
    })

    // 开始拖拽
    function startDrag() {
        isDragging.value = true
        document.addEventListener('mousemove', handleDrag)
        document.addEventListener('mouseup', stopDrag)
        document.body.style.cursor = 'ew-resize'
        document.body.style.userSelect = 'none'
    }

    // 停止拖拽
    function stopDrag() {
        isDragging.value = false
        document.removeEventListener('mousemove', handleDrag)
        document.removeEventListener('mouseup', stopDrag)
        document.body.style.cursor = ''
        document.body.style.userSelect = ''
        
        if (rafId) {
            cancelAnimationFrame(rafId)
            rafId = null
        }
    }

    // 清理函数
    function cleanup() {
        if (rafId) {
            cancelAnimationFrame(rafId)
        }
    }

    return {
        isDragging,
        sidebarWidth,
        containerRef,
        handleDrag,
        startDrag,
        stopDrag,
        cleanup
    }
} 