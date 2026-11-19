/**
 * Courseo SecureWatch Client-Side WebGL Player
 * - Prototype freezing for Canvas / MediaDevices / WebGL capture APIs
 * - GLSL anti-camcording temporal luminance modulation shader
 * - DevTools timing trap with session revocation callback
 * - Pseudo-random drifting forensic watermark overlay
 */

(function freezeCapturePrototypes() {
  'use strict';

  const lockMethod = (proto, prop) => {
    if (!proto || !(prop in proto)) return;
    try {
      Object.defineProperty(proto, prop, {
        value: proto[prop],
        writable: false,
        configurable: false,
      });
    } catch (_) {
      // Already non-configurable
    }
  };

  if (typeof HTMLCanvasElement !== 'undefined') {
    lockMethod(HTMLCanvasElement.prototype, 'toDataURL');
    lockMethod(HTMLCanvasElement.prototype, 'toBlob');
    lockMethod(HTMLCanvasElement.prototype, 'captureStream');
  }

  if (typeof CanvasRenderingContext2D !== 'undefined') {
    lockMethod(CanvasRenderingContext2D.prototype, 'getImageData');
  }

  if (typeof WebGLRenderingContext !== 'undefined') {
    lockMethod(WebGLRenderingContext.prototype, 'readPixels');
  }

  if (typeof navigator !== 'undefined' && navigator.mediaDevices) {
    const mdProto = Object.getPrototypeOf(navigator.mediaDevices);
    lockMethod(mdProto, 'getDisplayMedia');
    lockMethod(mdProto, 'getUserMedia');
    Object.freeze(navigator.mediaDevices);
  }
})();

export class CourseoSecurePlayer {
  constructor({ canvas, video, sessionToken, userId, onRevoke, refreshHz = 60 }) {
    this.canvas = canvas;
    this.video = video;
    this.sessionToken = sessionToken;
    this.userId = userId;
    this.onRevoke = onRevoke || (() => {});
    this.refreshHz = refreshHz;
    this.revoked = false;
    this.frameCount = 0;
    this.rafId = null;
    this.trapIntervalId = null;

    this.gl = this.canvas.getContext('webgl', {
      preserveDrawingBuffer: false,
      antialias: false,
    });

    if (!this.gl) {
      throw new Error('WebGL context unavailable');
    }

    this._initShaders();
    this._initWatermarkOverlay();
    this._startDevToolsTrap();
  }

  _initShaders() {
    const gl = this.gl;

    const vsSource = `
      attribute vec2 a_pos;
      attribute vec2 a_uv;
      varying vec2 v_uv;
      void main() {
        v_uv = a_uv;
        gl_Position = vec4(a_pos, 0.0, 1.0);
      }
    `;

    // Temporal luminance modulation targeting CMOS rolling-shutter aliasing
    const fsSource = `
      precision mediump float;
      uniform sampler2D u_frame;
      uniform float u_phase;
      uniform float u_amplitude;
      uniform float u_bands;
      varying vec2 v_uv;

      void main() {
        vec4 tex = texture2D(u_frame, v_uv);
        float band = sin((v_uv.y * u_bands * 6.2831853) + u_phase);
        float modFactor = 1.0 + (u_amplitude * band);
        gl_FragColor = vec4(clamp(tex.rgb * modFactor, 0.0, 1.0), tex.a);
      }
    `;

    const compile = (type, src) => {
      const shader = gl.createShader(type);
      gl.shaderSource(shader, src);
      gl.compileShader(shader);
      if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        throw new Error(gl.getShaderInfoLog(shader));
      }
      return shader;
    };

    const program = gl.createProgram();
    gl.attachShader(program, compile(gl.VERTEX_SHADER, vsSource));
    gl.attachShader(program, compile(gl.FRAGMENT_SHADER, fsSource));
    gl.linkProgram(program);
    gl.useProgram(program);

    const quad = new Float32Array([
      -1, -1, 0, 1,
       1, -1, 1, 1,
      -1,  1, 0, 0,
       1,  1, 1, 0,
    ]);

    const buf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.bufferData(gl.ARRAY_BUFFER, quad, gl.STATIC_DRAW);

    const aPos = gl.getAttribLocation(program, 'a_pos');
    const aUv = gl.getAttribLocation(program, 'a_uv');
    gl.enableVertexAttribArray(aPos);
    gl.vertexAttribPointer(aPos, 2, gl.FLOAT, false, 16, 0);
    gl.enableVertexAttribArray(aUv);
    gl.vertexAttribPointer(aUv, 2, gl.FLOAT, false, 16, 8);

    this.texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, this.texture);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);

    this.uniforms = {
      phase: gl.getUniformLocation(program, 'u_phase'),
      amplitude: gl.getUniformLocation(program, 'u_amplitude'),
      bands: gl.getUniformLocation(program, 'u_bands'),
    };
  }

  _initWatermarkOverlay() {
    const el = document.createElement('div');
    el.textContent = `${this.userId} | ${this.sessionToken.slice(0, 8)}`;
    Object.assign(el.style, {
      position: 'absolute',
      pointerEvents: 'none',
      userSelect: 'none',
      opacity: '0.18',
      color: '#ffffff',
      fontFamily: 'monospace',
      fontSize: '12px',
      textShadow: '0 0 2px #000',
      zIndex: '9999',
    });

    const parent = this.canvas.parentElement || document.body;
    parent.style.position = parent.style.position || 'relative';
    parent.appendChild(el);
    this.watermarkEl = el;
    this._seed = this._hashSeed(`${this.userId}:${this.sessionToken}`);
  }

  _hashSeed(str) {
    let h = 2166136261 >>> 0;
    for (let i = 0; i < str.length; i++) {
      h ^= str.charCodeAt(i);
      h = Math.imul(h, 16777619);
    }
    return h >>> 0;
  }

  _nextPrng() {
    // xorshift32
    let x = this._seed || 1;
    x ^= x << 13;
    x ^= x >>> 17;
    x ^= x << 5;
    this._seed = x >>> 0;
    return this._seed / 0xffffffff;
  }

  _updateWatermarkPosition() {
    if (this.frameCount % 90 !== 0) return;
    const xPct = 5 + this._nextPrng() * 75;
    const yPct = 5 + this._nextPrng() * 80;
    this.watermarkEl.style.left = `${xPct.toFixed(1)}%`;
    this.watermarkEl.style.top = `${yPct.toFixed(1)}%`;
  }

  _startDevToolsTrap() {
    this.trapIntervalId = setInterval(() => {
      if (this.revoked) return;
      const t0 = performance.now();
      // eslint-disable-next-line no-debugger
      debugger;
      const elapsed = performance.now() - t0;
      if (elapsed > 100) {
        this.revoke('devtools_timing_trap');
      }
    }, 2000);
  }

  start() {
    const render = () => {
      if (this.revoked) return;
      const gl = this.gl;

      if (this.video.readyState >= this.video.HAVE_CURRENT_DATA) {
        gl.bindTexture(gl.TEXTURE_2D, this.texture);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, this.video);

        // Alternate phase every frame to remain imperceptible to human vision
        // while inducing rolling-shutter banding on CMOS sensors
        const phase = (this.frameCount % 2 === 0) ? 0.0 : Math.PI;
        gl.uniform1f(this.uniforms.phase, phase);
        gl.uniform1f(this.uniforms.amplitude, 0.18);
        gl.uniform1f(this.uniforms.bands, this.refreshHz >= 120 ? 12.0 : 6.0);

        gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
      }

      this._updateWatermarkPosition();
      this.frameCount += 1;
      this.rafId = requestAnimationFrame(render);
    };

    this.rafId = requestAnimationFrame(render);
  }

  revoke(reason) {
    if (this.revoked) return;
    this.revoked = true;
    if (this.rafId) cancelAnimationFrame(this.rafId);
    if (this.trapIntervalId) clearInterval(this.trapIntervalId);
    if (this.video && typeof this.video.pause === 'function') {
      this.video.pause();
    }
    this.gl.clearColor(0, 0, 0, 1);
    this.gl.clear(this.gl.COLOR_BUFFER_BIT);
    this.onRevoke({ reason, sessionToken: this.sessionToken });
  }
}