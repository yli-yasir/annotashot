# Annotashot
Generate annotated screenshots. Suitable for app stores but may be used for different purposes.

## Samples
### Input:
<img src="https://github.com/yli-yasir/annotashot/blob/master/samples/screenshot.png?raw=true" width="300">

---

### Output:
<img src="https://github.com/yli-yasir/annotashot/blob/master/samples/a_create_counter.png?raw=true" width="300"> <img src="https://github.com/yli-yasir/annotashot/blob/master/samples/a_edit_counter.png?raw=true" width="300"> <img src="https://github.com/yli-yasir/annotashot/blob/master/samples/a_multiple_counters.png?raw=true" width="300">
<img src="https://github.com/yli-yasir/annotashot/blob/master/samples/a_animated_count.png?raw=true" width="300"> <img src="https://github.com/yli-yasir/annotashot/blob/master/samples/a_animations_sounds.png?raw=true" width="300"> <img src="https://github.com/yli-yasir/annotashot/blob/master/samples/a_count_history.png?raw=true" width="300"> 

---

### Example Command

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

---

### TODO
- [ ] Document command args.
- [ ] Add command arg validation.
- [ ] Support passing in either a font name or a font file path via the `annotation-font` option.
- [ ] Create release on GitHub.
- [ ] Publish on crates.io
  
