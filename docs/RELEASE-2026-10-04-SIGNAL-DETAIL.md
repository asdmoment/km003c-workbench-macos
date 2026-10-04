# KM003C 工作台 v0.1.0 (5) 本地交付

日期：2026-10-04。用户本轮要求：替换本地 App，并生成新版 DMG。沿用仓库的 Universal 打包链路，不发布到 GitHub。

后续用户授权公开发布同一 DMG，发布标签为 `v0.1.0-20261004`，完整中文更新说明见 [GitHub 发布说明](RELEASE-NOTES-2026-10-04.md)。上文“不发布到 GitHub”仅描述本地交付时的原始范围。

## 内容与边界

本版包含 [原始细节与界面重心第一阶段](UI-SIGNAL-DETAIL-IMPLEMENTATION-2026-10-04.md)：原始波形优先、全程/细节工作区、独立 U/|I|/|P|、局部/从零/锁定量程、选区原始统计和仪表栏减重。

保留当前源码中既有的 KM002C 兼容改动，不新增固件写入功能。不修改本次 UI 工作范围以外的 USB/PD、录制状态机、积分算法和 CSV/Parquet 23 列契约。

## 版本与构建

- App 版本：`0.1.0`，构建号：`5`；界面、Info.plist 和打包默认值一致。
- 目标架构：`arm64`、`x86_64`，合并为 Universal App。
- 构建命令：`APP_VERSION=0.1.0 APP_BUILD=5 SIGNING_MODE=adhoc CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true ./Scripts/build_release.sh`。
- 独立输出目录：`dist/signal-detail-20261004-build5/`；不覆盖其他历史交付目录。
- 签名方式：ad-hoc 本地签名，非 Apple Developer ID 公证。

## 已完成的检查

- 本轮版本调整后的格式检查、差异空白检查、Info.plist 校验通过。
- 发布版本规则的 Python 单元测试：3 项通过。
- UI 实现阶段完整工作区测试、严格 Clippy 和隔离演示窗口验收通过；详细结果和截图见实施说明。
- Apple Silicon 与 Intel 的 `--release --locked` 构建均通过；二进制合并后明确校验包含 `arm64`、`x86_64`。
- `hdiutil verify`、SHA-256 校验和 ad-hoc 签名严格验证通过。
- 只读挂载 DMG 后，镜像内 App 与打包 App 逐文件比对一致，构建号为 `5`。
- 本地安装 App 与打包 App 逐文件比对一致，签名和双架构检查通过。
- 实际从 `/Applications/KM003C 工作台.app` 启动，监控页正常响应；打开“设置 → 诊断与关于”，确认界面显示 `KM003C 工作台 v0.1.0 (5)`，随后关闭设置返回监控页。未发现真实设备，未启动真实录制。

## 交付文件

- [Universal DMG](../dist/signal-detail-20261004-build5/KM003C-Workbench-v0.1.0-macOS-universal.dmg)
- [SHA-256 校验文件](../dist/signal-detail-20261004-build5/KM003C-Workbench-v0.1.0-macOS-universal.dmg.sha256)
- 已安装：`/Applications/KM003C 工作台.app`。

DMG SHA-256：`a032fe941df58b86e04a331294337fac1b72705daa7084de124dbdab3daf3926`。

Universal 可执行文件 SHA-256：`2149c20a82f46c61722641d7ae14dc2054673489de0d31b8b3a8bbe4db817481`。DMG 内、打包目录和已安装 App 的内容一致。

## 安装与回退

开始构建时正式客户端已退出，未发现运行中的 `KM003CWorkbench` 进程。旧版 `0.1.0 (4)` 已完整备份并逐文件比对一致：

`release/rollback/signal-detail-20261004-build4/KM003C 工作台.app`

旧版可执行文件 SHA-256：`0c2fa72bd887f41192a4e8273cf30214dbf940aafd077010a884466a755d561b`。

安装前再次确认没有运行中的工作台进程。原安装 App 另行移至 `release/rollback/signal-detail-20261004-build4/Installed original.app`，可回退。

安装只替换 `/Applications/KM003C 工作台.app`，不清理或重置正式偏好、待恢复录制和用户导出的数据文件。验证期间没有切换语言、皮肤、采样率或录制规则；应用按现有机制自行管理正常启动状态。

打包结束后，仅将本轮输出目录中的 `build` 和 `dmg-staging` 中间目录移入废纸篓，可恢复：`/Users/xueweixun/.Trash/km003c-build5-packaging-intermediates-20261004/`。交付 App、DMG、SHA-256 和旧版备份保留。

## 尚未验证

真实 KM003C/KM002C 采样、与官方软件同数据同量程对比、USB 后台/锁屏续录和 PD 链路未在本轮执行。演示数据和本地包验证不等同于真机验收。
