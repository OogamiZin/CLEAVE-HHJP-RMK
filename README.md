# CLEAVE HHJP - RMK版ファームウェア

[CLEAVE HHJP](https://github.com/Rimebeck/CLEAVE-HHJP)（HHKB JP配列ベースの無線分割キーボード）用の [RMK](https://github.com/rmk-rs/rmk) ファームウェアです。
キーマップは [ZMK版のフォーク](https://github.com/OogamiZin/CLEAVE-HHJP) を移植し、接続先に応じた Windows / Mac レイヤーの自動切替を追加しています。

- RMK 0.9 / Seeed XIAO nRF52840 Plus x2（左 = central、右 = peripheral）/ BLE無線分割
- キーマップは [Vial](https://get.vial.today/) で変更可能（左側をUSB接続）
- GitHub Actions で左右の UF2 を自動ビルド

## ⚠️ 書き込む前に（ZMKとの行き来）

RMK は XIAO の SoftDevice（ZMK などが使う Nordic の BLE スタック）の領域を上書きします。
**一度 RMK を書き込んだ XIAO に通常の ZMK ファームを書き込んでも動きません。**

ZMK に戻すときは、[ZMK版フォーク](https://github.com/OogamiZin/CLEAVE-HHJP) の Actions が生成する
**`nosd` 付きの UF2**（SoftDevice なし版）を使ってください（`settings_reset` も nosd 版を使用）。

## ビルドと書き込み

1. `keyboard.toml` などを変更して main に push すると、Actions の **Build RMK firmware** が走ります
2. 成功した実行の **Artifacts → firmware** をダウンロードして展開
3. XIAO をブートローダーモード（リセットをダブルタップ）にして USB ドライブに UF2 をコピー
   - 左側: `cleave_hhjp_rmk_left_central.uf2`
   - 右側: `cleave_hhjp_rmk_right_peripheral.uf2`
4. 左右は自動でペアリングされます。PC 側に古い「CLEAVE HHJP」の Bluetooth 登録があれば削除してから接続

USB ケーブルを挿している間は USB 出力が優先されます（System レイヤーの `User8` で USB/BLE 切替）。

## レイヤー

| 番号 | 名前 | 内容 |
|---|---|---|
| 0 | Other0 | Windows / その他用ベース |
| 1 | Other1 | Fn（F1〜F12、音量、Home/End など） |
| 2 | Other2 | Nav（左手 ESDF 矢印・右手テンキー・Ctrl+Z/X/C/V/Y） |
| 3 | Mac0 | Mac 用ベース（Win/Alt 位置を Option/Cmd に入れ替え、右 Alt → 右 Cmd） |
| 4 | Mac1 | Mac 用 Fn（PrtSc → Cmd+Shift+4） |
| 5 | Mac2 | Mac 用 Nav（Cmd+Z/X/C/V、Redo は Cmd+Shift+Z、Home/End は Cmd+←/→） |
| 6 | System | Fn と Nav の同時押しで有効（下表） |

### System レイヤー（Fn + Nav を押しながら）

| キー | 動作 |
|---|---|
| 1〜5 | Bluetooth プロファイル BT1〜BT5（5秒長押しでそのプロファイルをクリアして再ペアリング） |
| BS | 現在のプロファイルをクリア |
| ← / → | 前 / 次のプロファイル |
| ¥ | 左右のペアリング情報をクリア（5秒長押し） |
| O | USB / BLE 出力切替 |
| W / M | ベースレイヤーを手動で Other0 / Mac0 に切替 |
| R | 再起動 |
| B | UF2 ブートローダーに入る |

ZMK版と同じく **Ctrl + Fn + F1〜F5** でもプロファイルを切り替えられ、**Ctrl + Fn + Del** で現在のプロファイルをクリアできます。

## Windows / Mac の自動切替

Bluetooth プロファイルが切り替わったとき（起動後の接続を含む）に、ベースレイヤーを自動で切り替えます。

| プロファイル | 既定の接続先 | ベースレイヤー |
|---|---|---|
| BT1 | Windows PC | Other0 |
| BT2 | MacBook | Mac0 |
| BT3 | iPad | Mac0 |
| BT4 / BT5 | その他 | Other0 |

割り当ては [`src/os_layer.rs`](src/os_layer.rs) の `PROFILE_IS_MAC` で変更できます。

仕組み: 左手マトリクスの空き位置 `(1,6)` / `(2,6)` に `DF(Other0)` / `DF(Mac0)` を割り当てた「仮想キー」を置き、
プロファイル変更時にプロセッサーがそれを押したことにしています。Vial には表示されませんが、
`keyboard.toml` でこの2つを変更すると自動切替が動かなくなります。

## 主なファイル

| ファイル | 内容 |
|---|---|
| `keyboard.toml` | マトリクス・ピン・キーマップ・BLE設定 |
| `vial.json` | Vial 用の物理レイアウト |
| `src/os_layer.rs` | Windows / Mac 自動切替 |
| `memory.x` | Adafruit nRF52 ブートローダー用メモリ配置 |

## ZMK版との違い

- バッテリー残量表示（rgbled-widget の LED 表示）はありません
- キーマップ変更ツールは ZMK Studio ではなく Vial
- Fn レイヤーの `*` `/` `+` `-` はテンキーのキーコードに変更（JIS 配列の PC でも記号がずれないように）
