# python/rust drawing experiments

This is an experiment with using [PyO3][] to share a drawing context from rust
to python, based on [druid][]. This is inspired by [drawbot][].

To use, run `cargo run file.py`, where `file.py` is a path to some file. At
runtime, we will watch this file for changes; if it is saved, the contents will
be loaded and executed. A `canvas` object will be added to the python namespace,
which allows the python code to draw no the screen. Currently the only method
exposed is `fill_rect(rect, color)`.

Here is a sample program:

```python
canvas.fill_rect(rect=(100, 20, 50, 50), color=(1.0, 1.0, 0.0, 1.0))
canvas.fill_rect(rect=(300, 20, 50, 50), color=(1.0, 1.0, 0.0, 1.0))
canvas.fill_rect(rect=(100, 250, 250, 20), color=(1.0, 1.0, 0.0, 1.0))
canvas.fill_rect(rect=(80, 230, 40, 20), color=(1.0, 1.0, 0.0, 1.0))
canvas.fill_rect(rect=(330, 230, 40, 20), color=(1.0, 1.0, 0.0, 1.0))
```

[PyO3]: https://github.com/PyO3/PyO3
[druid]: https://github.com/xi-editor/druid
[drawbot]: https://www.drawbot.com
