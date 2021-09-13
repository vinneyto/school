import jss from 'jss';
import {
  Camera,
  DirectionalLight,
  HemisphereLight,
  Mesh,
  MeshPhysicalMaterial,
  PerspectiveCamera,
  Scene,
  SphereBufferGeometry,
  WebGLRenderer,
} from 'three';

const { classes } = jss
  .createStyleSheet({
    arButton: {
      position: 'fixed',
      left: 'calc(50% - 50px)',
      top: 'calc(50% - 50px)',
    },
    arButtonHide: {
      display: 'none',
    },
  })
  .attach();

export async function demoARBall() {
  if (navigator.xr === undefined) {
    throw new Error('webxr is not supported');
  }
  const { xr } = navigator;
  const isArSessionSupported = await xr.isSessionSupported('immersive-ar');

  const xrButton = document.createElement('button');
  xrButton.innerHTML = 'Enter AR';
  xrButton.classList.add(classes.arButton);

  if (isArSessionSupported) {
    xrButton.addEventListener('click', activateXR);
    document.body.appendChild(xrButton);
  } else {
    onNoXRDevice();
  }

  let xrSession: XRSession;
  let canvas: HTMLCanvasElement;
  let gl: WebGLRenderingContext;
  let renderer: WebGLRenderer;
  let scene: Scene;
  let camera: Camera;
  let localReferenceSpace: XRReferenceSpace;

  async function activateXR() {
    xrSession = await xr.requestSession('immersive-ar');
    xrSession.addEventListener('end', onXRSessionEnd);

    initXRCanvas();

    await startXRSession();
  }

  function initXRCanvas() {
    canvas = document.createElement('canvas');
    document.body.appendChild(canvas);
    gl = canvas.getContext('webgl', {
      xrCompatible: true,
    }) as WebGLRenderingContext;

    if (gl === null) {
      throw new Error('enable to create context');
    }

    xrSession.updateRenderState({
      baseLayer: new XRWebGLLayer(xrSession, gl, {}),
      depthNear: 0.1,
      depthFar: 1000,
    });
  }

  async function startXRSession() {
    setXRButtonVisible(false);

    setupScene();

    localReferenceSpace = await xrSession.requestReferenceSpace('local');

    localReferenceSpace = localReferenceSpace.getOffsetReferenceSpace(
      // @ts-ignore
      new XRRigidTransform({ z: -3 })
    );

    xrSession.requestAnimationFrame(onXRFrame);
  }

  function onXRFrame(time: DOMHighResTimeStamp, frame: XRFrame) {
    xrSession.requestAnimationFrame(onXRFrame);

    const { baseLayer } = xrSession.renderState;

    if (baseLayer === undefined) {
      throw new Error('baseLayer is undefined');
    }

    const framebuffer = baseLayer.framebuffer;
    renderer.state.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);

    const pose = frame.getViewerPose(localReferenceSpace);
    if (pose) {
      const view = pose.views[0];

      const viewport = baseLayer.getViewport(view);
      renderer.setSize(viewport.width, viewport.height);

      camera.matrix.fromArray(view.transform.matrix);
      camera.projectionMatrix.fromArray(view.projectionMatrix);
      camera.updateMatrixWorld(true);

      camera.matrix.decompose(camera.position, camera.quaternion, camera.scale);

      console.log(camera.position);

      renderer.render(scene, camera);
    }
  }

  function setupScene() {
    renderer = new WebGLRenderer({
      alpha: true,
      preserveDrawingBuffer: true,
      canvas: canvas,
      context: gl,
    });

    renderer.autoClear = false;

    scene = createScene();

    camera = new PerspectiveCamera();
    camera.matrixAutoUpdate = false;
  }

  function createScene() {
    const scene = new Scene();

    scene.add(new HemisphereLight(0x606060, 0x404040));

    const light = new DirectionalLight(0xffffff);
    light.position.set(1, 1, 1).normalize();
    scene.add(light);

    const ball = new Mesh(
      new SphereBufferGeometry(0.2, 32, 32),
      new MeshPhysicalMaterial({ color: 'red', roughness: 0.2 })
    );

    // ball.position.y = 1;
    // ball.position.z = -3;

    scene.add(ball);

    return scene;
  }

  function onNoXRDevice() {
    setXRButtonVisible(false);
    document.body.innerHTML = 'There is no XR device';
  }

  function onXRSessionEnd() {
    setXRButtonVisible(true);
    xrSession.removeEventListener('end', onXRSessionEnd);
  }

  function setXRButtonVisible(value: boolean) {
    if (value) {
      xrButton.classList.remove(classes.arButtonHide);
    } else {
      xrButton.classList.add(classes.arButtonHide);
    }
  }
}
