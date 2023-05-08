# perception_eval_rs

A rust wrapper of [tier4/autoware_perception_evaluation](https://github.com/tier4/autoware_perception_evaluation).

## Support

- :heavy_check_mark: : Completed
- :white_check_mark: : WIP / TODO
- :x: : Not implemented yet

### Dataset

| Format   | Description                                          | Support            |
| :------- | :--------------------------------------------------- | :----------------- |
| NuScenes | [NuScenes format](https://www.nuscenes.org/nuscenes) | :heavy_check_mark: |
| NuImages | [NuImages format](https://www.nuscenes.org/nuimages) | :x:                |

### Evaluation tasks

| Task        | Metrics            | Description          | Support            |
| :---------- | :----------------- | :------------------- | :----------------- |
| Detection   | mAP, mAPH          | 3D detection         | :white_check_mark: |
| Tracking    | CLEAR              | 3D tracking          | :x:                |
| Prediction  | ADE, FDE, MissRate | 3D motion prediction | :x:                |
| Detection2D | mAP                | 2D detection         | :x:                |
| Tracking2D  | CLEAR              | 2D tracking          | :x:                |

### Matching objects

| Name             | Description                                  | Support            |
|:-----------------|:---------------------------------------------|:-------------------|
| `CenterDisntace` | Euclidean distance between center of objects | :heavy_check_mark: |
| `PlaneDistance`  | RMS score of nearest two planes              | :white_check_mark: |
| `Iou2D`          | 2D IoU score                                 | :white_check_mark: |
| `Iou3D`          | 3D IoU score                                 | :white_check_mark: |

### Filter objects

| Parameter           | Description                                                    | Support            |
|:--------------------|:---------------------------------------------------------------|:-------------------|
| `target_labels`     | List of labels to keep                                         | :heavy_check_mark: |
| `max_x_positions`   | List of maximum x positions for each label                     | :heavy_check_mark: |
| `max_y_positions`   | List of maximum y positions for each label                     | :heavy_check_mark: |
| `min_point_numbers` | List of minimum number of points the object's box must contain | :heavy_check_mark: |
| `target_uuids`      | List of instance IDs to keep                                   | :heavy_check_mark: |


## Documents

Run the following code to see the document.

```shell
$ cargo doc --open
```

## Examples

Run the following code to run examples.

```shell
# if you want see details
# $ export RUST_BACKTRACE=1 [or "full"]
$ cargo run --example <FILE_NAME> [-- <ARGUMENTS>]
```

## References

- [jerry73204/nuscenes-data-rs](https://github.com/jerry73204/nuscenes-data-rs)
