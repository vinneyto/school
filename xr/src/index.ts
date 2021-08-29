import jss from 'jss';
import preset from 'jss-preset-default';
import {
  Color,
  DirectionalLight,
  HemisphereLight,
  LineBasicMaterial,
  LineSegments,
  Mesh,
  MeshPhysicalMaterial,
  PerspectiveCamera,
  Scene,
  SphereBufferGeometry,
  sRGBEncoding,
  WebGLRenderer,
} from 'three';
import { BoxLineGeometry } from './common/BoxLineGeometries';
import { VRButton } from './common/VRButton';
import reset from './styles/reset';
import { styles } from './styles/style';

jss.setup(preset());
jss.createStyleSheet(reset).attach();

const { classes } = jss.createStyleSheet(styles).attach();

console.log(classes);

let scene: Scene;
let camera: PerspectiveCamera;
let renderer: WebGLRenderer;

function init() {
  scene = new Scene();
  scene.background = new Color(0x505050);

  camera = new PerspectiveCamera(
    50,
    window.innerWidth / window.innerHeight,
    0.1,
    10
  );
  camera.position.set(0, 1.6, 3);

  const room = new LineSegments(
    new BoxLineGeometry(6, 6, 6, 10, 10, 10),
    new LineBasicMaterial({ color: 0x808080 })
  );
  room.geometry.translate(0, 3, 0);
  scene.add(room);

  scene.add(new HemisphereLight(0x606060, 0x404040));

  const light = new DirectionalLight(0xffffff);
  light.position.set(1, 1, 1).normalize();
  scene.add(light);

  renderer = new WebGLRenderer({ antialias: true });
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.setSize(window.innerWidth, window.innerHeight);
  renderer.outputEncoding = sRGBEncoding;
  renderer.xr.enabled = true;
  document.body.appendChild(renderer.domElement);

  const ball = new Mesh(
    new SphereBufferGeometry(0.5, 32, 32),
    new MeshPhysicalMaterial({ color: 'red', roughness: 0.2 })
  );

  ball.position.y = 2;

  scene.add(ball);

  document.body.appendChild(VRButton.createButton(renderer));

  window.addEventListener('resize', onWindowResize);
}

function onWindowResize() {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();

  renderer.setSize(window.innerWidth, window.innerHeight);
}

function animate() {
  renderer.setAnimationLoop(render);
}

function render() {
  renderer.render(scene, camera);
}

init();
animate();
