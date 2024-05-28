Annotashot

```
cargo run --\
 --out-file-name out.png\
 --background-color FEFF86\
 --background-width 810\
 --background-height 1440\
 --screenshot-file-path ./samples/screenshot.png\
 --screenshot-resize-width 648\
 --screenshot-resize-height 1332\
 --screenshot-x 0.5\
 --screenshot-y 0.59\
 --screenshot-crop-top 0.03\
 --screenshot-crop-right 0\
 --screenshot-crop-bottom 0.04\
 --screenshot-crop-left 0\
 --annotation-text "Create lively\n counters"\
 --annotation-font "Permanent Marker"\
 --annotation-font-size 64\
 --annotation-font-color 000000\
 --annotation-x 0.5\
 --annotation-y 0.05
```

TODO:

- Consider making some arguments such as `screenshot-resize-width`
  `screenshot-resize-height` optional.

- Replace the `out-file-name` argument with `out-file-path`
