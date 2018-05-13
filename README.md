# vros-simulate
Simulator build for research of virtual reality optimization. The power consumption
constants is from [here](https://github.com/horizon-research/tx2-power-consumption).

## Usage
First of all, you need a recent version of rust compiler (maybe >= 1.25). For using
the simulator, you could try the instruction below or look into `auto_hit.sh` or
`auto_pc.sh`.

```
cargo run user_viewport_result/Paris-sJxiPiAaB4k tracedump/paris.txt object_as_cluster/paris.json 0.96 20 1600 1600 2000 2000
```

## Implementation Details
- Counting power consumption by using the hit rate calculated in the simulate
 function in `src/simulator.rs`

## Result
The data is visualized in the [vros-dataset](https://github.com/horizon-research/vros-dataset).
And you might be interested in the [summary](https://github.com/horizon-research/vros-dataset/blob/master/heatmap.ipynb) of the data.

## Related repositories
- [power consumption constants](https://github.com/horizon-research/tx2-power-consumption)
- [data visualization](https://github.com/horizon-research/vros-dataset)