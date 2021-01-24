import { WasmApp, WasmRenderer } from './pkg/index'
import { audioManager } from './audio'

const CHANNEL_COUNT = 3

const AUDIO_ASSETS = [
  'assets/audio/bgm',
  'assets/audio/graze',
  'assets/audio/kill',
  'assets/audio/title'
]
const ENABLE_AUDIO = 'assets/audio/toggle_sound'

const ICON_SOUND_ON = 'assets/images/sound_on.svg'
const ICON_SOUND_OFF = 'assets/images/sound_off.svg'

const CANVAS_ID = 'mycanvas'

window.play_se = function play_se(channel, filename) {
  audioManager.playSe(channel, filename)
}

window.play_loop = function play_loop(channel, filename) {
  audioManager.playLoop(channel, filename)
}


function fitCanvas() {
  const canvas = document.getElementById(CANVAS_ID)
  if (canvas.width >= window.innerWidth || canvas.height >= window.innerHeight) {
    if (canvas.width >= window.innerWidth) {
      canvas.style.width = `100%`
      canvas.style.height = `${canvas.height * window.innerWidth / canvas.width}px`
    }
    if (canvas.height >= window.innerHeight) {
      canvas.style.height = `100%`
      canvas.style.width = `${canvas.width * window.innerHeight / canvas.height}px`
    }
  } else {
    canvas.style.width = `${canvas.width}px`
    canvas.style.height = `${canvas.height}px`
  }
}

function setupSoundButton() {
  const toggleSound = () => {
    audioManager.toggleEnabled()
    if (audioManager.enabled)
      audioManager.playSe(0, ENABLE_AUDIO)
    document.getElementById('sound-icon').src = audioManager.enabled ? ICON_SOUND_ON : ICON_SOUND_OFF
  }
  const soundIconHolder = document.getElementById('sound-icon-holder')
  soundIconHolder.addEventListener('click', toggleSound)
}

function setupResizeListener() {
  window.addEventListener('resize', (_) => {
    fitCanvas()
  })
}

function createCoverScreen(title) {
  const cover = document.createElement('div')
  cover.className = 'centering'
  cover.style.position = 'absolute'
  cover.style.left = cover.style.top = cover.style.right = cover.style.bottom = '0'
  cover.style.backgroundColor = 'rgba(0,0,0,1)'
  cover.style.color = 'white'
  cover.style.textAlign = 'center'
  cover.innerText = title

  document.body.appendChild(cover)
  return cover
}

fitCanvas()
setupResizeListener()

const renderer = WasmRenderer.new(CANVAS_ID)
const app = WasmApp.new(renderer,
  function get_now() {
    return performance.now()
  })

document.addEventListener('keydown', (event) => {
  app.on_key(event.code, true)
})
document.addEventListener('keyup', (event) => {
  app.on_key(event.code, false)
})

const loop = (function () {
  const target_fps = 60
  const ticks = 1000 / target_fps
  const max_skip = 5
  const margin = ticks / 8

  let prev = performance.now()
  return function loop() {
    const now = performance.now()
    let n = Math.floor((now - prev + margin) / ticks)
    if (n > 0) {
      if (n <= max_skip) {
        prev += n * ticks
      } else {
        n = max_skip
        prev = now
      }
      for (let i = 0; i < n; ++i)
        app.update()
      app.draw()
    }
    requestAnimationFrame(loop)
  }
})()

const cover = createCoverScreen('Loading...')
audioManager.createContext(CHANNEL_COUNT)
audioManager.loadAllAudios(AUDIO_ASSETS)
  .then(() => {
    document.body.removeChild(cover)
    showInfoBlock()
    setupSoundButton()
    requestAnimationFrame(loop)
  })

  function showInfoBlock() {
    var element = document.getElementById("info");
    element.style.display = 'block';
  }