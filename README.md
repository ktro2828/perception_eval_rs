# perception_eval_rs

A rust wrapper of [tier4/autoware_perception_evaluation](https://github.com/tier4/autoware_perception_evaluation).

## Support

- :heavy_check_mark: : Completed
- :white_check_mark: : WIP / TODO
- :x: : Not implemented yet

### Dataset

| Format   | Description                                          | Support            |
| :------- | :--------------------------------------------------- | :----------------- |
| NuScenes | [NuScenes format](https://www.nuscenes.org/nuscenes) | :white_check_mark: |
| NuImages | [NuImages format](https://www.nuscenes.org/nuimages) | :x:                |

### Evaluation tasks

| Task        | Metrics            | Description          | Support            |
| :---------- | :----------------- | :------------------- | :----------------- |
| Detection   | mAP, mAPH          | 3D detection         | :white_check_mark: |
| Tracking    | CLEAR              | 3D tracking          | :x:                |
| Prediction  | ADE, FDE, MissRate | 3D motion prediction | :x:                |
| Detection2D | mAP                | 2D detection         | :x:                |
| Tracking2D  | CLEAR              | 2D tracking          | :x:                |

## References

- [jerry73204/nuscenes-data-rs](https://github.com/jerry73204/nuscenes-data-rs)