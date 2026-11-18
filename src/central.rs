#![no_main]
#![no_std]

mod os_layer;

use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard_central {
    /// 接続中のBluetoothプロファイルに応じてベースレイヤー（Other0 / Mac0）を自動で切り替える
    #[register_processor(event)]
    fn os_layer_switcher() -> crate::os_layer::OsLayerSwitcher {
        crate::os_layer::OsLayerSwitcher::new()
    }
}
