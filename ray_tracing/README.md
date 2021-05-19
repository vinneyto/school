# How to run

    cargo run --bin one_weekend --release
    cargo run --bin one_weekend --features="precise" --release
    
    cargo run --bin one_weekend_vk --release

    cargo run --bin next_week --release
    cargo run --bin next_week --features="precise" --release

# How to extreact data from three mesh

```javascript
const extract = (mesh) => {
  const g = mesh.geometry;
  const data = {};
  Object.keys(g.attributes).forEach((key) => {
    const attribute = g.attributes[key];
    data[key] = [...attribute.array];
  });
  data.index = [...g.index.array];
  return data;
};
```

![monkey_diffuse](https://github.com/vinneyto/school/blob/main/ray_tracing/monkey_diffuse.jpg?raw=true)
