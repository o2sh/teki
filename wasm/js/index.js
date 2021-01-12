import {WasmApp, WasmRenderer} from '../pkg/index'
import {audioManager} from './audio'

const CHANNEL_COUNT = 3

const CANVAS_ID = 'mycanvas'

window.play_se = function play_se(channel, filename) {
  audioManager.playSe(channel, filename)
}

function fitCanvas() {
  const canvas = document.getElementById(CANVAS_ID)
  if (canvas.width / canvas.height >= window.innerWidth / window.innerHeight) {
    canvas.style.width = `100%`
    canvas.style.height = `${canvas.height * window.innerWidth / canvas.width}px`
  } else {
    canvas.style.height = `100%`
    canvas.style.width = `${canvas.width * window.innerHeight / canvas.height}px`
  }
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
  cover.style.backgroundColor = 'rgba(0,0,0,0.5)'
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
},)

document.addEventListener('keydown', (event) => {
  app.on_key(event.code, true)
})
document.addEventListener('keyup', (event) => {
  app.on_key(event.code, false)
})

const loop = (function() {
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

    requestAnimationFrame(loop)