import jss from 'jss';
import preset from 'jss-preset-default';
import { demoARBall } from './demo/demoARBall';
import { demoVRBall } from './demo/demoVRBall';
import { demoWebGPU } from './demo/demoWebGPU';
import reset from './styles/reset';
import { styles } from './styles/style';

jss.setup(preset());
jss.createStyleSheet(reset).attach();

const { classes } = jss.createStyleSheet(styles).attach();

interface Demo {
  name: string;
  start: () => void;
}

const demos: Demo[] = [
  { name: 'vr-ball', start: demoVRBall },
  { name: 'ar-ball', start: demoARBall },
  { name: 'webgpu-demo', start: demoWebGPU },
];

if (window.location.pathname === '/') {
  for (const demo of demos) {
    const link = document.createElement('a');
    link.href = `/${demo.name}`;
    link.innerHTML = demo.name;
    link.className = classes.link;
    document.body.appendChild(link);
  }
} else {
  const demo = demos.find((d) => `/${d.name}` === window.location.pathname);
  if (demo !== undefined) {
    demo.start();
  } else {
    document.body.innerHTML = 'unable to find demo!!!';
  }
}
