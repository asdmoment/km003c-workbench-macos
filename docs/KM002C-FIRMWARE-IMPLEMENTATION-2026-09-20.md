# KM002C 兼容与固件升级实施记录

## 当前结果

2026-09-20 实机 USB 描述符：POWER-Z KM002C，VID 5FC9，PID 0061。原实现仅筛选 KM003C 的 0063。

已增加 0061 精确匹配，保留 0063；USB reset 后按原 PID 与序列号重新打开，避免切换到另一机型。未知 PID 不纳入。应用已按设备返回型号显示连接状态，无需伪装为 KM003C。

厂商接口探针读取成功：型号 KM002C、固件 2.0.0、streaming permission=true。2/10/50/1000 SPS 请求后分别收到 2/7/35/63 个点；这是短时非空响应验证，不是实际速率、完整性或长录制验收。HID 接口因 Busy 未验证。未执行重置、校准写入、进入 DFU、擦除或固件写入。

探针：`cargo run --locked -p km003c-lib --example meter_compatibility -- --vendor`。仅在没有其他采集任务时运行，它会短时启动/停止各档数据流。

## 模块与实施顺序

1. 连接层按明确 USB 身份支持两机型，测量/录制/导出复用现有链路。
2. 回归 KM002C 长录制、四档实际速率、PD、离线导入；未验证能力不宣称通用。
3. 独立固件包解析器：真实格式、目标机型/硬件版本、完整性与官方来源校验。不能按文件名或自行生成的 SHA-256 判断固件可信。
4. 独立升级 worker：独占设备，禁止与录制/离线下载并行；准备、验证包、进入升级模式、擦除、分块写入、设备校验、重启与读取版本。
5. 未经确认的 bootloader 地址、擦除范围和写入命令不发送。未知状态不得自动重复擦除；中断后按真实恢复协议处理，不能承诺必然回滚。
6. UI 仅在完整升级链路可用后开放实际写入，展示目标机型、当前/目标版本、进度及原始错误。拒绝跨机型包，不用占位按钮冒充已实现功能。

## 官方证据与当前缺口

- [技术支持及固件/客户端入口](https://www.chargerlab.com/km003c-km002c-technical-support/)
- [官方升级教程](https://www.chargerlab.com/how-to-update-the-firmware-of-chargerlab-power-z-km002c-pd3-1-tester/)
- 官方支持页链接的 `hiddemo_vs2019_for-KM002C3C.zip` 提供共享 HID 测量协议。公开示例有 JumpDfu 命令标识，不等于完整刷写协议。
- 官方直链 `https://chargerlab.oss-cn-shenzhen.aliyuncs.com/POWER-Z.zip` 实际包含 Setup.exe 和说明，未包含独立固件文件。
- 官方固件 Google Drive 入口已找到，尚未取得可解析的目标固件及完整擦除/写入/校验规则。

2026-09-20 已构建并安装桌面 v0.1.0 (4) 到 `/Applications/KM003C 工作台.app`。Universal App 包含 arm64/x86_64，DMG 挂载、SHA-256 和 ad-hoc 签名校验通过；安装包与本地 App 逐文件一致。不是公证版本。

安装后实际界面识别 `KM002C · 设备采样中`，50 SPS 下点数从 127 增至 5345，实时电流和功率读数持续变化。未执行长录制、PD 或离线导入验收。固件写入仍未实现。保留 CSV/Parquet 和录制状态机不变。

- 安装包：`dist/km002c-20260920/KM003C-Workbench-v0.1.0-macOS-universal.dmg`
- SHA-256：`e5c03e9509be0bb7df412ec6a1daa13353006d482140bce5a8074524d2673bd5`
- 旧版备份：`release/rollback/km002c-20260920-build3/KM003C 工作台.app`
