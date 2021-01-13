class AudioManager {
  constructor(channelCount) {
    this.audios = {}
    this.audioLoadings = {}
    this.enabled = false
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
    } else if (!(filename in this.audioLoadings)) {
      this.loadAudio(filename)
        .then(() => this.playSe(channel, filename))
        .catch(err => console.error(`Audio eror: ${err}`))
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

  loadAllAudios(filenames) {
    return Promise.all(filenames.map((filename) => {
      return this.loadAudio(filename)
    }))
  }

  loadAudio(filename) {
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
