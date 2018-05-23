# vros-simulate
Simulator build for research of virtual reality optimization. The power consumption
constants is from [here](https://github.com/horizon-research/tx2-power-consumption).

## TODO
- Client-side Optimizations
    - Make use of **threshold**
        - lower threshold means client side need to draw something by guessing
        - to make threshold lower (~= 0.8), we need to draw more precisely, therefore:
        - [ ] (static) find frame (closest in time) that covers the missing part
        - [ ] (dynamic) use motion vector to draw the missing part

## Usage
First of all, you need a recent version of rust compiler (maybe >= 1.25).
For using the simulator, you need a `user viewport file` and a `tracedump file` and these two are the first argument and second
argument that the simulator need to run.

The next two argument is the `threshold` and `segment size` which is simple, but notice that the threshold is `a floating point >= 0 && <= 1`.

The next four argument is the width and height for level one and level two viewport respectively. And if the level one and level two viewport
has the same dimension (ex: 2000 1000 2000 1000) then the simulator simulate with only level one frame and the full size frame.

The last two argument is power and O0, which is the content to dump and the flag of optimization level, but now the simulator only support with `power O0`.

To conclude, you can simply change the dimension in following line:
```bash
cargo run user_viewport_result/Elephant-training-2bpICIClAIg tracedump/elephant.txt object_as_cluster/elephant.json 0.96 20 1440 1440 1440 1440 power O0 >> elephant_power.txt &
cargo run user_viewport_result/Rhino-training-7IWp875pCxQ tracedump/rhinos.txt object_as_cluster/rhinos.json 0.96 20 1440 1440 1440 1440 power O0 >> rhino_power.txt &
cargo run user_viewport_result/Rollercoaster-8lsB-P8nGSM tracedump/roller.txt object_as_cluster/roller.json 0.96 20 1440 1440 1440 1440 power O0 >> roller_power.txt &
cargo run user_viewport_result/Timelapse-CIw8R8thnm8 tracedump/nyc.txt object_as_cluster/nyc.json 0.96 20 1440 1440 1440 1440 power O0 >> nyc_power.txt &
cargo run user_viewport_result/Paris-sJxiPiAaB4k tracedump/paris.txt object_as_cluster/paris.json 0.96 20 1440 1440 1440 1440 power O0 >> paris_power.txt &
```

## Implementation Details
- TODO: should fill this part as detail as possible!
- Counting power consumption by using the hit rate calculated in the simulate
 function in `src/simulator.rs`

## Results
The data is visualized in the [vros-dataset](https://github.com/horizon-research/vros-dataset).
And you might be interested in the [summary](https://github.com/horizon-research/vros-dataset/blob/master/heatmap.ipynb) of the data.

## Related repositories
- [power consumption constants](https://github.com/horizon-research/tx2-power-consumption)
- [data visualization](https://github.com/horizon-research/vros-dataset)