import jss from 'jss';
import preset from 'jss-preset-default';
import { VRBallDemo } from './demo/VRBallDemo';
import reset from './styles/reset';
import { styles } from './styles/style';

jss.setup(preset());
jss.createStyleSheet(reset).attach();

const { classes } = jss.createStyleSheet(styles).attach();

interface Demo {
  name: string;
  ctor: new () => void;
}

const demos: Demo[] = [{ name: 'vr-ball', ctor: VRBallDemo }];

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
    new demo.ctor();
  } else {
    document.body.innerHTML = 'unable to find demo!!!';
  }
}
