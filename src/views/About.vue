<template>
    <div class="heading">
        <span class="clip">Vice City Multiplayer Launcher V2</span>
        <span>{{ $t('about.built_by') }}</span>
        <span>{{ $t('about.features') }}</span>
    </div>
</template>

<style scoped>
.heading {
    line-height: 64px;
    font-size: 48px;
    display: flex;
    flex-direction: column;
    justify-content: center;
}
.clip {
    background: linear-gradient(
        45deg,
        var(--main-color) 15%,
        var(--dark-main-color) 50%
    ); /*设置渐变的方向从左到右 颜色从ff0000到ffff00*/
    background-clip: text; /*将设置的背景颜色限制在文字中*/
    -webkit-text-fill-color: transparent; /*给文字设置成透明*/
}
</style>
