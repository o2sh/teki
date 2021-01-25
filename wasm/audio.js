class AudioManager {
  constructor(channelCount) {
    this.audios = {}
    this.audioLoadings = {}
    this.enabled = true
    this.count = 0
  }

  createContext(channelCount) {
    let audioContext = window.AudioContext || window.webkitAudioContext
    this.context = new audioContext()

    this.channels = new Array(channelCount)
  }

  toggleEnabled() {
    this.enabled = !this.enabled
    if (!this.enabled)
      this.stopAll()
  }


  playSe(channel, filename) {
    this.playMusic(channel, filename, false)
  }

  playLoop(channel, filename) {
    this.playMusic(channel, filename, true)
  }

  playMusic(channel, filename, isLoop) {
    if (!this.enabled)
      return

    if (filename in this.audios) {
      if (channel < this.channels.length) {
        if (this.channels[channel] != null) {
          this.channels[channel].stop()
        }

        const source = this.context.createBufferSource()
        source.connect(this.context.destination)
        this.channels[channel] = source

        source.buffer = this.audios[filename]
        source.start(0)
        if (isLoop) {
          source.loop = true
        }
      }
    }
  }

  stopAll() {
    for (let ch = 0; ch < this.channels.length; ++ch) {
      this.stop(ch)
    }
  }

  stop(channel) {
    const source = this.channels[channel]
    if (source != null) {
      source.stop()
      this.channels[channel] = null
    }
  }

  loadAllAudios(filenames, cover) {
    return Promise.all(filenames.map((filename) => {
      return this.loadAudio(filename, filenames.length, cover)
    }))
  }

  loadAudio(filename, length, cover) {
    return new Promise((resolve, reject) => {
      this.audioLoadings[filename] = true

      const path = `${filename}.mp3`
      const request = new XMLHttpRequest()
      request.open('GET', path, true)
      request.responseType = 'arraybuffer'

      request.onload = () => {
        this.context.decodeAudioData(
          request.response,
          (buffer) => {
            this.count++
            cover.innerText = `Loading ${this.count}/${length}`
            this.audios[filename] = buffer
            resolve(true)
          },
          (err) => {
            reject(err)
          }
        )
      }
      request.onerror = (_) => {
        reject(_)
      }
      request.send()
    })
  }
}

export const audioManager = new AudioManager()
