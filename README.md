## 概要

流体の2DシミュレーションをRustで行うデモ。

Material Point Method(MPM)を用いて計算している。

## 実行方法

```sh
cargo run --release
```

## 操作方法

* 左クリック長押し: 流体を動かす。

## 使用ライブラリ

* **[Bevy](https://github.com/bevyengine/bevy)**: ECSによる計算、描画用。

## 参考リポジトリ、文献

* **[Incremental MPM](https://github.com/nialltl/incremental_mpm)**: Unityによる実装。
* **[High-Performance MLS-MPM Solver with Cutting and Coupling (CPIC) (MIT License)](https://github.com/yuanming-hu/taichi_mpm)**: Taichiライブラリを利用した簡潔な実装。
* Jiang, Chenfanfu, et al. "The affine particle-in-cell method." ACM Transactions on Graphics (TOG) 34.4 (2015): 1-10.
* Hu, Yuanming, et al. "A moving least squares material point method with displacement discontinuity and two-way rigid body coupling." ACM Transactions on Graphics (TOG) 37.4 (2018): 1-14.

## TODO

* ステップ実行の実装。
* 並列化する。
* WebAssembly化し、ブラウザで閲覧可能にする。
* 計算精度の向上(圧力計算)。
