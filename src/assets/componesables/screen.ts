import { ref, Ref } from 'vue';
import { WindowSize } from '../../types';

const windowSize = ref<WindowSize>({
    width: 0,
    height: 0,
});

const observer = new ResizeObserver(() => {
    windowSize.value.width = window.innerWidth;
    windowSize.value.height = window.innerHeight;
});
observer.observe(document.body);

export function getWindowSize(): Ref<WindowSize> {
    return windowSize;
}
