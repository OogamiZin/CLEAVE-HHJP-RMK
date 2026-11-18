//! 接続先に応じたベースレイヤーの自動切替
//!
//! Bluetoothプロファイルが切り替わったとき（起動時の最初の接続も含む）に、
//! そのプロファイルが Mac 用かどうかでベースレイヤーを Other0 / Mac0 に切り替える。
//!
//! RMK 0.9 にはプロセッサーから直接デフォルトレイヤーを変更するAPIがないため、
//! 左手マトリクスの空き位置 (1,6) / (2,6) を「仮想キー」として使い、
//! そこに割り当てた DF(Other0) / DF(Mac0) を押したことにして切り替えている。

use embassy_time::Timer;
use rmk::event::{ConnectionStatusChangeEvent, KeyboardEvent, publish_event_async};
use rmk::macros::processor;

/// Bluetoothプロファイル（BT1〜BT5）ごとの接続先。true = Mac / iPad、false = Windows / その他
/// 接続先の割り当てを変えたいときはここを編集する。
const PROFILE_IS_MAC: [bool; 5] = [
    false, // BT1: Windows PC
    true,  // BT2: MacBook
    true,  // BT3: iPad
    false, // BT4: その他
    false, // BT5: その他
];

/// keyboard.toml で DF(Other0) を割り当てた仮想キーの位置
const VKEY_OTHER: (u8, u8) = (1, 6);
/// keyboard.toml で DF(Mac0) を割り当てた仮想キーの位置
const VKEY_MAC: (u8, u8) = (2, 6);

#[processor(subscribe = [ConnectionStatusChangeEvent])]
pub struct OsLayerSwitcher {
    last_profile: Option<u8>,
}

impl OsLayerSwitcher {
    pub fn new() -> Self {
        Self { last_profile: None }
    }

    async fn on_connection_status_change_event(&mut self, event: ConnectionStatusChangeEvent) {
        let profile = event.0.ble.profile;
        if self.last_profile == Some(profile) {
            return;
        }
        self.last_profile = Some(profile);

        let is_mac = PROFILE_IS_MAC.get(profile as usize).copied().unwrap_or(false);
        let (row, col) = if is_mac { VKEY_MAC } else { VKEY_OTHER };
        publish_event_async(KeyboardEvent::key(row, col, true)).await;
        Timer::after_millis(20).await;
        publish_event_async(KeyboardEvent::key(row, col, false)).await;
    }
}
